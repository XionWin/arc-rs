use core::Vertex2;
use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::{
    def::{Point, PointFlag},
    FillState, Primitive, StrokeState,
};

pub trait VectorShape: Debug {
    fn get_stroke_primitive(&self) -> Primitive;
    fn get_fill_primitive(&self) -> Primitive;
}

impl<T: core::Shape + ?Sized> VectorShape for T {
    fn get_stroke_primitive(&self) -> Primitive {
        get_stroke_primitive(self.get_commands(), self.get_style())
    }

    fn get_fill_primitive(&self) -> Primitive {
        get_fill_primitive(self.get_commands(), self.get_style())
    }
}

fn get_stroke_primitive(_commands: &[core::Command], _style: &core::Style) -> Primitive {
    Primitive::new(Box::new([]), Box::new(StrokeState::new(1f32)))
}

fn get_fill_primitive(commands: &[core::Command], _style: &core::Style) -> Primitive {
    let _is_closed = commands.iter().any(|x| x == &core::Command::Close);
    // util::print_debug!("is_closed: {}", is_closed);
    let _points = get_points(commands);
    // util::print_debug!("points: {:#?}", points);

    let x = 100;
    let y = 100;
    let width = 100;
    let height = 100;
    let vec = vec![
        Vertex2::new(x, y, 0.5f32, 1f32),
        Vertex2::new(x, y + height, 0.5f32, 1f32),
        Vertex2::new(x + width, y + height, 0.5f32, 1f32),
        Vertex2::new(x + width, y, 0.5f32, 1f32),
    ];
    Primitive::new(
        Box::<[core::Vertex2]>::from(vec),
        Box::new(FillState::default()),
    )
}

fn get_points(commands: &[core::Command]) -> Option<Point> {
    let mut first_core_point = match commands.get(0) {
        Some(x) => to_start_point(x),
        None => None,
    };

    let first_point = match &mut first_core_point {
        Some(first_core_point) => {
            let first_point = Rc::new(RefCell::new(Point::new_from_point(
                first_core_point,
                PointFlag::CORNER,
            )));
            match commands.get(1..) {
                Some(commands) => {
                    let mut last_point = first_point.clone();
                    for command in commands {
                        let core_points = to_points(command, last_point.borrow().get_point_ref());
                        for (index, core_point) in core_points.iter().enumerate() {
                            let point = Rc::new(RefCell::new(Point::new_from_point(
                                core_point,
                                if index + 1 == core_points.len() {
                                    PointFlag::CORNER
                                } else {
                                    PointFlag::NONE
                                },
                            )));
                            point.borrow_mut().set_previous(Rc::downgrade(&last_point));
                            last_point.borrow_mut().set_next(point);
                            let temp = last_point.borrow_mut().next().unwrap();
                            last_point = temp;
                        }
                    }
                }
                None => {}
            }
            Some((*first_point).replace(Point::new(0f32, 0f32, PointFlag::NONE)))
        }
        None => None,
    };
    match &first_point {
        Some(point) => util::print_debug!("first_point: {}", point),
        None => {}
    }
    first_point
}

fn to_start_point(command: &core::Command) -> Option<core::Point<f32>> {
    match command {
        core::Command::MoveTo(point0) | core::Command::LineTo(point0) => Some(point0.clone()),
        _ => None,
    }
}

fn to_points(command: &core::Command, point0: &core::Point<f32>) -> Vec<core::Point<f32>> {
    match command {
        core::Command::MoveTo(point1) | core::Command::LineTo(point1) => vec![point1.clone()],
        core::Command::BezierTo(point1, point2, point3) => get_bezier_points(
            point0,
            point1,
            point2,
            point3,
            crate::parameter::TESS_TOL,
            0,
        ),
        core::Command::Close => Vec::new(),
    }
}

fn get_bezier_points(
    point0: &core::Point<f32>,
    point1: &core::Point<f32>,
    point2: &core::Point<f32>,
    point3: &core::Point<f32>,
    tess_tol: f32,
    level: u8,
) -> Vec<core::Point<f32>> {
    let mut result = Vec::new();
    if level <= 10 {
        let point01 = point0.get_center_point(point1);
        let point12 = point1.get_center_point(point2);
        let point23 = point2.get_center_point(point3);
        let point012 = point01.get_center_point(&point12);

        let dx = point3.x - point0.x;
        let dy = point3.y - point0.y;
        let d2 = ((point1.x - point3.x) * dy - (point1.y - point3.y) * dx).abs();
        let d3 = ((point2.x - point3.x) * dy - (point2.y - point3.y) * dx).abs();

        if (d2 + d3) * (d2 + d3) < tess_tol * (dx * dx + dy * dy) {
            result.push(point3.clone());
        } else {
            let point123 = point12.get_center_point(&point23);
            let point0123 = point012.get_center_point(&point123);

            result.splice(
                result.len()..,
                get_bezier_points(point0, &point01, &point012, &point0123, tess_tol, level + 1),
            );
            result.splice(
                result.len()..,
                get_bezier_points(&point0123, &point123, &point23, point3, tess_tol, level + 1),
            );
        }
    }
    result
}
