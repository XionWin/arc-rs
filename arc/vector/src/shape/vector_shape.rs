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
                    Some(point0) => get_bezier_points(point0, point1, point2, point3),
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
    point0: &core::Point<f32>,
    point1: &core::Point<f32>,
    point2: &core::Point<f32>,
    point3: &core::Point<f32>,
) -> Vec<Point> {
    todo!()
}
