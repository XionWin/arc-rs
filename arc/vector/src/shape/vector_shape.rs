use core::Vertex2;
use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::{
    def::{Point, PointFlag},
    FillState, Primitive, StrokeState,
};

use super::CommandPoint;

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
    let (points, is_closed) = crate::CommandCalculator::to_points(commands);

    let first_point = Rc::new(RefCell::new(Point::new_from_command_point(
        &points.first().expect("first core_point can't be null"),
        PointFlag::CORNER,
    )));

    match points.get(1..) {
        Some(command_points) => {
            let last_point = first_point.clone();
            attach_point(last_point.clone(), command_points, is_closed);
        }
        None => {}
    }

    util::print_debug!("first_point: {}", first_point.borrow());

    Some((first_point, is_closed))
}

fn attach_point(
    first_point: Rc<RefCell<Point>>,
    command_points: &[CommandPoint<f32>],
    is_closed: bool,
) {
    let mut last_point = first_point.clone();
    for command_point in command_points.iter() {
        let point = Rc::new(RefCell::new(Point::new_from_command_point(
            command_point,
            if command_point.is_corner {
                PointFlag::CORNER
            } else {
                PointFlag::NONE
            },
        )));
        update_chain_calculate_data(&mut point.borrow_mut(), &mut last_point.borrow_mut());
        point.borrow_mut().set_previous(Rc::downgrade(&last_point));
        last_point.borrow_mut().set_next(point);
        let temp = last_point.borrow_mut().next().unwrap();
        last_point = temp;
    }

    if is_closed {
        first_point
            .borrow_mut()
            .set_previous(Rc::downgrade(&last_point));
        update_chain_calculate_data(&mut first_point.borrow_mut(), &mut last_point.borrow_mut());
    }
}

fn update_chain_calculate_data(curr: &mut Point, prev: &mut Point) {
    // update dx dy len
    update_point_data_by_next(prev, &curr);
    // update dmx dmy dmr2
    update_point_data_by_previous(curr, &prev);
}

fn update_point_data_by_next(curr: &mut Point, next: &Point) {
    let mut dx = next.point.x - curr.point.x;
    let mut dy = next.point.y - curr.point.y;
    let len = (dx.powi(2) + dy.powi(2)).sqrt();
    if len > 0f32 {
        let i_len = 1.0f32 / len;
        dx = dx * i_len;
        dy = dy * i_len;
    }
    dx = if dx == 0f32 { 0f32 } else { dx };
    dy = if dy == 0f32 { 0f32 } else { dy };

    curr.dx = Some(dx);
    curr.dy = Some(dy);
    curr.len = Some(len);
}

fn update_point_data_by_previous(curr: &mut Point, prev: &Point) {
    let dlx0 = prev.point.x;
    let dly0 = -prev.point.y;
    let dlx1 = curr.point.x;
    let dly1 = -curr.point.y;
    let mut dmx = (dlx0 + dlx1) * 0.5f32;
    let mut dmy = (dly0 + dly1) * 0.5f32;

    let dmr2 = dmx.powi(2) + dmy.powi(2);
    if dmr2 > 0.1e-6f32 {
        let mut i_scale = 1.0f32 / dmr2;
        if i_scale > 600.0f32 {
            i_scale = 600.0f32;
        }
        dmx = dmx * i_scale;
        dmy = dmy * i_scale;
    }
    dmx = if dmx == 0f32 { 0f32 } else { dmx };
    dmy = if dmy == 0f32 { 0f32 } else { dmy };

    curr.dmx = Some(dmx);
    curr.dmy = Some(dmy);
    curr.dmr2 = Some(dmr2);
}
