mod node;
mod pattern;

use aoi_components::{method::Method, request::range::RangeList};
use crate::{result::Result, handler::Handler};
use self::{node::Node, pattern::Pattern};

#[allow(non_snake_case)]
pub(crate) struct Router {
    GET:    Node,
    POST:   Node,
    PATCH:  Node,
    DELETE: Node,
} impl Router {
    pub(crate) fn new() -> Self {
        Self {
            GET:    Node::new(Pattern::Nil),
            POST:   Node::new(Pattern::Nil),
            PATCH:  Node::new(Pattern::Nil),
            DELETE: Node::new(Pattern::Nil),
        }
    }

    pub(crate) fn register(&mut self,
        method:       Method,
        path_pattern: &'static str /* already validated */,
        handler:      Handler,
    ) -> std::result::Result<(), String> {
        let err_msg = format!(
            "path pattern `{}` is resistred duplicatedly",
            if path_pattern == "" {"/"} else {path_pattern}
        );

        let mut path = path_pattern.split('/');
        { path.next(); }

        let tree = match method {
            Method::GET    => &mut self.GET,
            Method::POST   => &mut self.POST,
            Method::PATCH  => &mut self.PATCH,
            Method::DELETE => &mut self.DELETE,
        };
        
        tree.register(path, handler, err_msg)
    }

    pub(crate) fn search<'req>(&self,
        method:       Method,
        request_path: &'req str,
    ) -> Result<(
        &Handler,
        RangeList,
    )> {
        let mut path = request_path.split('/');
        { path.next(); }

        let offset = method.len();

        match method {
            Method::GET    => &self.GET,
            Method::POST   => &self.POST,
            Method::PATCH  => &self.PATCH,
            Method::DELETE => &self.DELETE,
        }.search(
            path, RangeList::new(), offset
        )
    }
}