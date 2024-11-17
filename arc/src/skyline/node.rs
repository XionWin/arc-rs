use core::Location;
use std::cmp;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Node {
    pub(crate) location: Location,
    pub(crate) width: i32,
}

impl Node {
    pub fn new(location: Location, width: i32) -> Self {
        Node { location, width }
    }

    pub fn split(&mut self, width: i32) -> Self {
        self.width -= width;
        Node {
            location: Location::new(self.location.x + width, self.location.y),
            width,
        }
    }

    pub fn fuse(&mut self, node: &Node, threshold: i32) {
        if (self.location.y - node.location.y).abs() > threshold {
            panic!("The difference bwteen the two nodes is large than threshold")
        }
        if self.location.x + self.width != node.location.x {
            panic!("The two nodes is not adjacent")
        }

        self.location.y = cmp::max(self.location.y, node.location.y);
        self.width += node.width;
    }
}
