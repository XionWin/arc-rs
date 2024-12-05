pub struct CommandCalculator {}

impl CommandCalculator {
    pub fn to_points(commands: &[core::Command]) -> (Vec<Vec<core::Point<f32>>>, bool) {
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
}
