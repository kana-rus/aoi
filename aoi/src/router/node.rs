use std::str::Split;

use crate::result::Result;

use super::pattern::Pattern;

pub(crate) struct Node {
    pub(crate) pattern:  Pattern,
    pub(crate) handler:  Option<HandleFunc>,
    pub(crate) before:   Vec<BeforeMiddeware>,
    pub(crate) after:    Vec<AfterMiddleware>,
    pub(crate) children: Vec<Node>,
} impl Node {
    pub(crate) fn new(pattern: Pattern) -> Self {
        Self {
            pattern,
            handler:  None,
            before:   Vec::new(),
            after:    Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn search<'tree, 'req>(&'tree self,
        mut path:     Split<'req, char>,
        mut params:   RangeList,
        mut read_pos: usize,
    ) -> Result<(
        &'tree HandleFunc,
        RangeList,
        &'tree Vec<BeforeMiddleware>,
        &'tree Vec<AfterMiddleware>,
    )> {
        if let Some(section) = path.next() {
            read_pos += 1 /*'/'*/;
            if let Some(child) = 'search: {
                for child in &self.children {
                    if child.pattern.matches(section) {
                        if child.pattern.is_param() {
                            let range = BufRange::new(read_pos + 1, read_pos + section.len());
                            tracing::debug!("path param: `{}`", section);
                            params.push(range)?;
                        }
                        // for proc in &self.before {before.push(proc)}
                        // for proc in &self.after {after.push(proc)}
                        break 'search Some(child)
                    }
                }
                None

            } {
                child.search(path, params, read_pos + section.len(), )//before, after)

            } else {
                Err(Response::NotFound(None))
            }
            
        } else {
            Ok((
                self.handler.as_ref()._else(|| Response::NotFound(None))?,
                params,
                &self.before,
                &self.after,
            ))
        }
    }

    pub(super) fn register_handler(&mut self,
        mut path: Split<'static, char>,
        handler:  HandleFunc,
        err_msg:  String,
    ) -> std::result::Result<(), String> {
        if let Some(section) = path.next() {
            let pattern = Pattern::from(section);
            if let Some(child) = 'search: {
                for child in &mut self.children {
                    if child.pattern.is(&pattern) {
                        break 'search Some(child)
                    }
                }
                None
            } {
                child.register_handler(path, handler, err_msg)

            } else {
                let mut new_branch = Node::new(pattern);
                new_branch.attach(path, handler);
                self.children.push(new_branch);
                Ok(())
            }

        } else {
            if self.pattern.is_nil() {
                self.handler = Some(handler);
                Ok(())
            } else {
                Err(err_msg)
            }
        }
    }
    fn attach(&mut self,
        path:    Split<'static, char>,
        handler: HandleFunc,
    ) {
        let path = path.rev().collect::<Vec<_>>();
        self._attach(path, handler)
    }
    fn _attach(&mut self,
        mut path: Vec<&'static str>,
        handler:  HandleFunc,
    ) {
        if let Some(section) = path.pop() {
            let mut new_node = Node::new(Pattern::from(section));
            new_node._attach(path, handler);
            self.children.push(new_node)
        } else {
            self.handler = Some(handler)
        }
    }

    pub(super) fn register_before_middleware(mut self,
        route: &'static str /* already validated */,
        mut store: BeforeMiddlewareStore,
        err_msg:  String,
        // warn_msg: String,
    ) -> std::result::Result<Self, String> {
        if route == "*" {
            self.apply_before_to_me_and_all_child(store, err_msg)?;
        } else if route.ends_with("/*") {
            let mut route = route.trim_end_matches("/*").split('/');
            { route.next(); }

            if let Some(root) = self.search_apply_root(route) {
                for child in &mut root.children {
                    store = child.apply_before_to_me_and_all_child(store, err_msg.clone())?
                }
            } else {
                // tracing::warn!(warn_msg)
            }
        } else {
            let mut route = route.split('/');
            { route.next(); }

            if let Some(target) = self.search_apply_root(route) {
                target.before.push(
                    store.pop().ok_or(err_msg)?
                )
            } else {
                // tracing::warn!(warn_msg)
            }
        }

        Ok(self)
    }
    pub(super) fn register_after_middleware(mut self,
        route: &'static str /* already validated */,
        mut store: AfterMiddlewareStore,
        err_msg:  String,
        // warn_msg: String,
    ) -> std::result::Result<Self, String> {
        if route == "*" {
            self.apply_after_to_me_and_all_child(store, err_msg)?;
        } else if route.ends_with("/*") {
            let mut route = route.trim_end_matches("/*").split('/');
            { route.next(); }

            if let Some(root) = self.search_apply_root(route) {
                for child in &mut root.children {
                    store = child.apply_after_to_me_and_all_child(store, err_msg.clone())?
                }
            } else {
                // tracing::warn!(warn_msg)
            }
        } else {
            let mut route = route.split('/');
            { route.next(); }

            if let Some(target) = self.search_apply_root(route) {
                target.after.push(store.pop().ok_or(err_msg)?)
            } else {
                // tracing::warn!(warn_msg)
            }
        }

        Ok(self)
    }

    fn search_apply_root(&mut self, mut path: Split<'static, char>) -> Option<&mut Self> {
        if let Some(section) = path.next() {
            if let Some(child) = 'search: {
                for child in &mut self.children {
                    if child.pattern.matches(section) {
                        break 'search Some(child)
                    }
                }
                None

            } {
                child.search_apply_root(path)

            } else {
                None
            }
        } else {
            Some(self)
        }
    }

    fn apply_before_to_me_and_all_child(&mut self,
        mut store: BeforeMiddlewareStore,
        err_msg:   String,
    ) -> std::result::Result<BeforeMiddlewareStore, String> {
        self.before.push(store.pop().ok_or_else(|| err_msg.clone())?);

        for child in &mut self.children {
            store = child.apply_before_to_me_and_all_child(store, err_msg.clone())?
        }

        Ok(store)
    }
    fn apply_after_to_me_and_all_child(&mut self,
        mut store: AfterMiddlewareStore,
        err_msg:   String,
    ) -> std::result::Result<AfterMiddlewareStore, String> {
        self.after.push(store.pop().ok_or_else(|| err_msg.clone())?);

        for child in &mut self.children {
            store = child.apply_after_to_me_and_all_child(store, err_msg.clone())?
        }

        Ok(store)
    }
}