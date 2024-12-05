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
    let _points = get_point_chain(commands);
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

fn get_point_chain(commands: &[core::Command]) -> Option<(Rc<RefCell<Point>>, bool)> {
    let (point_lists, is_closed) = crate::CommandCalculator::to_points(commands);

    let first_point = Rc::new(RefCell::new(Point::new_from_point(
        &point_lists.first().expect("first core_point can't be null")[0],
        PointFlag::CORNER,
    )));

    match point_lists.get(1..) {
        Some(core_points) => {
            let mut last_point = first_point.clone();
            for core_points in core_points {
                last_point = attach_point(last_point, core_points);
            }

            if is_closed {
                first_point
                    .borrow_mut()
                    .set_previous(Rc::downgrade(&last_point));
            }
        }
        None => {}
    }

    util::print_debug!("first_point: {}", first_point.borrow());

    Some((first_point, is_closed))
}

fn attach_point(
    last_point: Rc<RefCell<Point>>,
    core_points: &[core::Point<f32>],
) -> Rc<RefCell<Point>> {
    let mut last_point = last_point;
    for (index, core_point) in core_points.iter().enumerate() {
        let point = Rc::new(RefCell::new(Point::new_from_point(
            core_point,
            if index + 1 == core_points.len() {
                PointFlag::CORNER
            } else {
                PointFlag::NONE
            },
        )));
        update_chain_calculate_data(&mut point.borrow_mut(), &mut last_point.borrow());
        point.borrow_mut().set_previous(Rc::downgrade(&last_point));
        last_point.borrow_mut().set_next(point);
        let temp = last_point.borrow_mut().next().unwrap();
        last_point = temp;
    }
    last_point
}

fn update_chain_calculate_data(curr: &mut Point, prev: &Point) {}
