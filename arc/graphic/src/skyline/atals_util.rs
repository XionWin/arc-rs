use core::{Location, Rect};

use crate::skyline::node::Node;

use super::atlas::Atlas;

pub(crate) fn get_best_location(atlas: &Atlas, rect: &Rect<i32>) -> Option<Location<i32>> {
    let mut best_node = Option::<Node>::None;

    for node in &atlas.nodes {
        let node_location = node.location;
        let node_width = node.width;
        if node_width >= rect.size.width {
            if best_node.is_none() {
                best_node = Some(*node);
            } else {
                let best_node_value = best_node.unwrap();
                if (node_location.y < best_node_value.location.y)
                    || (node_location.y == best_node_value.location.y
                        && node.width < best_node_value.width)
                {
                    best_node = Some(*node);
                }
            }
        }
    }
    best_node.map(|x| x.location)
}
