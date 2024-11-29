use core::Vertex2;
use std::fmt::Debug;

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
    let is_closed = commands.iter().any(|x| x == &core::Command::Close);
    util::print_debug!("is_closed: {}", is_closed);
    let points = get_points(commands);
    util::print_debug!("points: {:#?}", points);

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

fn get_points(commands: &[core::Command]) -> Box<[Point]> {
    let mut last_point: Option<&core::Point<f32>> = Option::<&core::Point<f32>>::None;
    let vec: Vec<Point> = commands
        .iter()
        .flat_map(|x| match x {
            core::Command::MoveTo(point) => {
                last_point = Some(point);
                vec![Point::new(point.x, point.y, PointFlag::CORNER)]
            }
            core::Command::LineTo(point) => {
                last_point = Some(point);
                vec![Point::new(point.x, point.y, PointFlag::CORNER)]
            }
            core::Command::BezierTo(point1, point2, point3) => {
                let result = match last_point {
                    Some(point0) => get_bezier_points(
                        point0.x,
                        point0.y,
                        point1.x,
                        point1.y,
                        point2.x,
                        point2.y,
                        point3.x,
                        point3.y,
                        1.0f32,
                        PointFlag::CORNER,
                        0,
                    ),
                    None => util::print_panic!("vector_shape bezier start point is None"),
                };
                last_point = Some(point3);
                result
            }
            core::Command::Close => Vec::new(),
        })
        .collect();
    Box::<[Point]>::from(vec)
}

fn get_bezier_points(
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    x3: f32,
    y3: f32,
    x4: f32,
    y4: f32,
    tess_tol: f32,
    point_flag: PointFlag,
    level: u8,
) -> Vec<Point> {
    let mut result = Vec::new();
    if level <= 10 {
        let x12 = (x1 + x2) * 0.5f32;
        let y12 = (y1 + y2) * 0.5f32;
        let x23 = (x2 + x3) * 0.5f32;
        let y23 = (y2 + y3) * 0.5f32;
        let x34 = (x3 + x4) * 0.5f32;
        let y34 = (y3 + y4) * 0.5f32;
        let x123 = (x12 + x23) * 0.5f32;
        let y123 = (y12 + y23) * 0.5f32;

        let dx = x4 - x1;
        let dy = y4 - y1;
        let d2 = ((x2 - x4) * dy - (y2 - y4) * dx).abs();
        let d3 = ((x3 - x4) * dy - (y3 - y4) * dx).abs();

        if (d2 + d3) * (d2 + d3) < tess_tol * (dx * dx + dy * dy) {
            result.push(Point::new(x4, y4, point_flag));
        } else {
            let x234 = (x23 + x34) * 0.5f32;
            let y234 = (y23 + y34) * 0.5f32;
            let x1234 = (x123 + x234) * 0.5f32;
            let y1234 = (y123 + y234) * 0.5f32;

            result.splice(
                ..,
                get_bezier_points(
                    x1,
                    y1,
                    x12,
                    y12,
                    x123,
                    y123,
                    x1234,
                    y1234,
                    tess_tol,
                    PointFlag::NONE,
                    level + 1,
                ),
            );
            result.splice(
                ..,
                get_bezier_points(
                    x1234,
                    y1234,
                    x234,
                    y234,
                    x34,
                    y34,
                    x4,
                    y4,
                    tess_tol,
                    point_flag,
                    level + 1,
                ),
            );
        }
    }
    result
}
