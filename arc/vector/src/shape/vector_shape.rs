use core::Vertex2;
use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::{
    def::{Point, PointFlag},
    FillState, Primitive, StrokeState,
};

pub trait VectorShape: Debug {
    fn get_stroke_primitive(&self) -> Option<Primitive>;
    fn get_fill_primitive(&self) -> Option<Primitive>;
}

impl<T: core::Shape + ?Sized> VectorShape for T {
    fn get_stroke_primitive(&self) -> Option<Primitive> {
        get_stroke_primitive(self)
    }

    fn get_fill_primitive(&self) -> Option<Primitive> {
        get_fill_primitive(self)
    }
}

fn get_stroke_primitive<T>(shape: &T) -> Option<Primitive>
where
    T: core::Shape + ?Sized,
{
    Some(Primitive::new(
        Box::new([]),
        Box::new(Into::<StrokeState>::into(shape.get_style())),
        shape.get_rectangle(),
    ))
}

fn get_fill_primitive<T>(shape: &T) -> Option<Primitive>
where
    T: core::Shape + ?Sized,
{
    let commands = shape.get_commands();
    let _is_closed = commands.iter().any(|x| x == &core::Command::Close);
    // util::print_debug!("is_closed: {}", is_closed);
    let points = get_points(commands);
    // util::print_debug!("points: {:#?}", points);

    let vertices = match points {
        Some(points) => Some(
            points
                .iter()
                .map(|point| Vertex2::new(point.x, point.y, 0.5f32, 1f32))
                .collect::<Vec<Vertex2>>(),
        ),
        None => None,
    };

    let fill_state = FillState::new(
        Into::<core::Paint>::into(shape.get_style().get_background()),
        core::Matrix2D::default(),
    );

    match vertices {
        Some(vertices) => Some(Primitive::new(
            Box::<[core::Vertex2]>::from(vertices),
            Box::new(Into::<FillState>::into(fill_state)),
            shape.get_rectangle(),
        )),
        None => None,
    }
}

fn get_points(commands: &[core::Command]) -> Option<Vec<Point>> {
    let is_closed = commands.last() == Some(&core::Command::Close);
    match crate::CommandCalculator::to_points(commands) {
        Some(command_points) => {
            let points = {
                let mut point_0 = Option::<Rc<RefCell<Point>>>::None;
                let points = command_points
                    .iter()
                    .map(|command_point| {
                        let point_1 = Rc::new(RefCell::new(Point::new_from_command_point(
                            command_point,
                            if command_point.is_corner {
                                PointFlag::CORNER
                            } else {
                                PointFlag::NONE
                            },
                        )));
                        if let Some(point_0) = point_0.clone() {
                            whirling_update_point(
                                &mut point_0.borrow_mut(),
                                &mut point_1.borrow_mut(),
                            );
                        }
                        point_0 = Some(point_1.clone());
                        point_1
                    })
                    .collect::<Vec<_>>();

                if is_closed {
                    whirling_update_point(
                        &mut points.last().unwrap().borrow_mut(),
                        &mut points.first().unwrap().borrow_mut(),
                    );
                }
                points
            };

            whirling_update_reversed(&points, is_closed);

            let points = points
                .iter()
                .map(|cell| Point::new_from_point(&cell.borrow()))
                .collect::<Vec<Point>>();

            // for point in &points {
            //     util::print_debug!("{}", point);
            // }
            Some(points)
        }
        None => None,
    }
}

fn whirling_update_reversed(points: &[Rc<RefCell<Point>>], is_closed: bool) {
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
            &mut points.first().unwrap().borrow_mut(),
            &mut points.last().unwrap().borrow_mut(),
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
