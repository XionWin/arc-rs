pub struct CommandCalculator {}

impl CommandCalculator {
    pub fn to_points(commands: &[core::Command]) -> (Vec<Vec<core::Point<f32>>>, bool) {
        if !Self::check_commands(commands) {
            util::print_panic!("command check error")
        }
        let mut previous_command = Option::<&core::Command>::None;
        let points = commands
            .iter()
            .map(|command| {
                let points = command.to_points(previous_command, crate::parameter::TESS_TOL);
                previous_command = Some(command);
                points
            })
            .collect();

        let is_closed = commands.last() == Some(&core::Command::Close);
        (points, is_closed)
    }

    fn check_commands(commands: &[core::Command]) -> bool {
        Self::check_commands_first_move(commands) && Self::check_commands_last_may_close(commands)
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
}
