use crate::CommandPoint;
use core::Pt;

pub struct CommandCalculator {}

impl CommandCalculator {
    pub fn to_points(commands: &[core::Command]) -> (Vec<CommandPoint<f32>>, bool) {
        if !check_commands(commands) {
            util::print_panic!("command check error")
        }

        let is_closed = commands.last() == Some(&core::Command::Close);

        let mut previous_command = Option::<&core::Command>::None;
        let points = commands
            .iter()
            .flat_map(|command| {
                let points = command.to_points(previous_command, crate::parameter::TESS_TOL);
                let command_points = points
                    .iter()
                    .enumerate()
                    .map(|(i, point)| CommandPoint::new_from_point(point, i + 1 == points.len()))
                    .collect::<Vec<CommandPoint<f32>>>();
                previous_command = Some(command);
                command_points
            })
            .collect::<Vec<CommandPoint<f32>>>();

        (optimize_points(points, is_closed), is_closed)
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
