use core::Vertex2;
use std::{
    cell::RefCell,
    fmt::Debug,
    rc::{Rc, Weak},
};

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

fn get_point_chain(commands: &[core::Command]) -> Option<Rc<RefCell<Point>>> {
    let mut first_core_point = match commands.get(0) {
        Some(x) => x.to_start_point(),
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
                        let core_points = command.to_points(
                            last_point.borrow().get_point_ref(),
                            crate::parameter::TESS_TOL,
                        );
                        last_point = attach_point(last_point, &core_points);
                    }
                    match commands.last() {
                        Some(last_command) if last_command == &core::Command::Close => first_point
                            .borrow_mut()
                            .set_previous(Rc::downgrade(&last_point)),
                        _ => {}
                    }
                }
                None => {}
            }
            Some(first_point)
        }
        None => None,
    };
    match &first_point {
        Some(point) => util::print_debug!("first_point: {}", point.borrow()),
        None => {}
    }
    first_point
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
        point.borrow_mut().set_previous(Rc::downgrade(&last_point));
        last_point.borrow_mut().set_next(point);
        // modify_point(&mut last_point.borrow_mut());
        let temp = last_point.borrow_mut().next().unwrap();
        last_point = temp;
    }
    last_point
}

fn update_chain_calculate_data(curr: &mut Point, prev: &Point) {}
