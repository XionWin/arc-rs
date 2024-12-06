use core::Vertex2;
use std::{cell::RefCell, fmt::Debug};

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

fn get_points(commands: &[core::Command]) -> Option<(Vec<Point>, bool)> {
    let (command_points, is_closed) = crate::CommandCalculator::to_points(commands);

    let cell_points = command_points
        .iter()
        .map(|command_point| {
            RefCell::new(Point::new_from_command_point(
                command_point,
                if command_point.is_corner {
                    PointFlag::CORNER
                } else {
                    PointFlag::NONE
                },
            ))
        })
        .collect::<Vec<_>>();

    whirling_update(&cell_points, is_closed);

    let points = cell_points
        .iter()
        .map(|cell| Point::new_from_point(&cell.borrow()))
        .collect::<Vec<Point>>();

    for point in &points {
        util::print_debug!("{}", point);
    }

    Some((points, is_closed))
}

fn whirling_update(points: &[RefCell<Point>], is_closed: bool) {
    {
        let mut point_0 = points.get(0).unwrap().borrow_mut();
        for index in 1..points.len() {
            let mut point_1 = points.get(index).unwrap().borrow_mut();
            whirling_update_point(&mut point_0, &mut point_1);
            point_0 = point_1
        }
    }
    {
        if is_closed {
            whirling_update_point(
                &mut points.get(points.len() - 1).unwrap().borrow_mut(),
                &mut points.get(0).unwrap().borrow_mut(),
            );
        }
    }
    {
        let mut point_1 = points.get(points.len() - 1).unwrap().borrow_mut();
        let max_index = points.len() - 2;
        for index in 0..=max_index {
            let mut point_0 = points.get(max_index - index).unwrap().borrow_mut();
            whirling_update_point_reversed(&mut point_1, &mut point_0);
            point_1 = point_0;
        }
    }
    if is_closed {
        whirling_update_point_reversed(
            &mut points.get(0).unwrap().borrow_mut(),
            &mut points.get(points.len() - 1).unwrap().borrow_mut(),
        );
    }
}

fn whirling_update_point(curr: &mut Point, next: &Point) {
    let mut dx = next.x - curr.x;
    let mut dy = next.y - curr.y;
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

fn whirling_update_point_reversed(curr: &mut Point, prev: &Point) {
    let dlx0 = prev.dy.unwrap();
    let dly0 = -prev.dx.unwrap();
    let dlx1 = curr.dy.unwrap();
    let dly1 = -curr.dx.unwrap();
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
