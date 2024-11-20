use core::{Location, Rect, Size};

use super::node::Node;
use crate::skyline::atals_util;

pub struct Atlas {
    pub size: Size<i32>,
    pub(crate) nodes: Vec<Node>,
}

impl Atlas {
    pub fn new(size: Size<i32>) -> Self {
        Self {
            size,
            nodes: vec![Node::new(Location::new(0, 0), size.width)],
        }
    }

    pub fn add_rect(&self, rect: &Rect<i32>) -> Location<i32> {
        let best_location = atals_util::get_best_location(self, rect);
        match best_location {
            Some(x) => x,
            None => self.nodes.first().unwrap().location,
        }
    }
}
