use crate::CommandPoint;
use core::Pt;

pub struct CommandCalculator {}

impl CommandCalculator {
    pub fn to_points(commands: &[core::Command]) -> Option<Vec<CommandPoint<f32>>> {
        if !check_commands(commands) {
            util::print_warning!("command check error");
            None
        } else {
            let is_closed = commands.last() == Some(&core::Command::Close);

            let mut command_0 = Option::<&core::Command>::None;
            let points = commands
                .iter()
                .flat_map(|command_1| {
                    let points = command_1.to_points(command_0, crate::parameter::TESS_TOL);
                    let command_points = points
                        .iter()
                        .enumerate()
                        .map(|(i, point)| {
                            CommandPoint::new_from_point(point, i + 1 == points.len())
                        })
                        .collect::<Vec<CommandPoint<f32>>>();
                    command_0 = Some(command_1);
                    command_points
                })
                .collect::<Vec<CommandPoint<f32>>>();

            Some(optimize_points(enforce_winding(points), is_closed))
        }
    }
}

fn check_commands(commands: &[core::Command]) -> bool {
    check_commands_first_move(commands) && check_commands_last_may_close(commands)
}

fn check_commands_first_move(commands: &[core::Command]) -> bool {
    matches!(commands.first(), Some(core::Command::MoveTo(_)))
        && !match commands.get(1..) {
            Some(command) => command
                .iter()
                .any(|command| matches!(command, core::Command::MoveTo(_))),
            None => false,
        }
}

fn check_commands_last_may_close(commands: &[core::Command]) -> bool {
    !match commands.get(0..commands.len() - 1) {
        Some(command) => command
            .iter()
            .any(|command| matches!(command, core::Command::Close)),
        None => false,
    }
}

fn enforce_winding(points: Vec<CommandPoint<f32>>) -> Vec<CommandPoint<f32>> {
    let mut points = points;
    match get_area(&points) {
        Some(area) => {
            if area < 0f32 {
                points.reverse();
            }
        }
        None => {}
    }
    points
}

fn get_area(points: &Vec<CommandPoint<f32>>) -> Option<f32> {
    let mut area = 0f32;
    if points.len() > 2 {
        for index in 2..points.len() {
            let a = &points[0];
            let b = &points[index - 1];
            let c = &points[index];

            let abx = b.x - a.x;
            let aby = b.y - a.y;
            let acx = c.x - a.x;
            let acy = c.y - a.y;
            area += (acx * aby - abx * acy) / 2f32;
        }
        Some(area)
    } else {
        None
    }
}

fn optimize_points(points: Vec<CommandPoint<f32>>, is_closed: bool) -> Vec<CommandPoint<f32>> {
    let mut points = points;
    if is_closed
        && points.first().unwrap().get_distance(points.last().unwrap()) < crate::parameter::DIS_TOL
    {
        points.remove(points.len() - 1);
        let idx = points.len() - 1;
        points[idx].is_corner = true;
    }

    let mut last_point = CommandPoint::new_from_point(&core::Point::new(f32::MIN, f32::MIN), false);
    let new_points = points
        .iter()
        .filter(|&point| {
            if point.get_distance(&last_point) > crate::parameter::DIS_TOL {
                last_point = *point;
                true
            } else {
                false
            }
        })
        .map(|point| *point)
        .collect::<Vec<CommandPoint<f32>>>();

    new_points
}
