use crate::{Point, Pt};

#[derive(Debug, PartialEq)]
pub enum Command {
    MoveTo(Point<f32>),
    LineTo(Point<f32>),
    BezierTo(Point<f32>, Point<f32>, Point<f32>),
    Close,
}

impl Command {
    pub fn get_end_point(&self) -> &Point<f32> {
        match self {
            Command::MoveTo(point) => point,
            Command::LineTo(point) => point,
            Command::BezierTo(_, _, point) => point,
            Command::Close => util::print_panic!("can't get point form command::colse"),
        }
    }

    pub fn to_points(&self, previous: Option<&Command>, tess_tol: f32) -> Vec<Point<f32>> {
        match self {
            Command::MoveTo(point1) | Command::LineTo(point1) => vec![point1.clone()],
            Command::BezierTo(point1, point2, point3) => Self::get_bezier_points(
                previous
                    .expect("command::bezierto must have the preview command")
                    .get_end_point(),
                point1,
                point2,
                point3,
                tess_tol,
                0,
            ),
            Command::Close => Vec::new(),
        }
    }

    fn get_bezier_points(
        point0: &Point<f32>,
        point1: &Point<f32>,
        point2: &Point<f32>,
        point3: &Point<f32>,
        tess_tol: f32,
        level: u8,
    ) -> Vec<Point<f32>> {
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
                    Self::get_bezier_points(
                        point0,
                        &point01,
                        &point012,
                        &point0123,
                        tess_tol,
                        level + 1,
                    ),
                );
                result.splice(
                    result.len()..,
                    Self::get_bezier_points(
                        &point0123,
                        &point123,
                        &point23,
                        point3,
                        tess_tol,
                        level + 1,
                    ),
                );
            }
        }
        result
    }
}
