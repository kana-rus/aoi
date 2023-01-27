use std::str::Split;
use aoi_components::{
    request::range::{RangeList, BufRange},
    response::{Response, Status},
    result::ElseResponse,
};

use crate::{result::Result, handler::Handler};
use super::pattern::Pattern;

pub(crate) struct Node {
    pub(crate) pattern:  Pattern,
    pub(crate) handler:  Option<Handler>,
    pub(crate) children: Vec<Node>,
} impl Node {
    pub(crate) fn new(pattern: Pattern) -> Self {
        Self {
            pattern,
            handler:  None,
            children: Vec::new(),
        }
    }

    pub fn search<'tree, 'req>(&'tree self,
        mut path:     Split<'req, char>,
        mut params:   RangeList,
        mut read_pos: usize,
    ) -> Result<(
        &'tree Handler,
        RangeList,
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
                child.search(path, params, read_pos + section.len())

            } else {
                Err(Response {
                    status: Status::NotFound,
                    additional_headers: String::new(),
                    body:   None,
                })
            }
            
        } else {
            Ok((
                self.handler.as_ref()._else(|| Response {
                    status: Status::NotFound,
                    additional_headers: String::new(),
                    body:   None,
                })?,
                params,
            ))
        }
    }

    pub(super) fn register(&mut self,
        mut path: Split<'static, char>,
        handler:  Handler,
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
                child.register(path, handler, err_msg)

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
        handler: Handler,
    ) {
        let path = path.rev().collect::<Vec<_>>();
        self._attach(path, handler)
    }
    fn _attach(&mut self,
        mut path: Vec<&'static str>,
        handler:  Handler,
    ) {
        if let Some(section) = path.pop() {
            let mut new_node = Node::new(Pattern::from(section));
            new_node._attach(path, handler);
            self.children.push(new_node)
        } else {
            self.handler = Some(handler)
        }
    }
}