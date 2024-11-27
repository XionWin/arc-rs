use std::borrow::Borrow;

use crate::{FillState, State, StrokeState};

#[test]
pub fn test() {
    let state: Box<dyn State> = Box::new(crate::StrokeState::new(1.0f32));
    let state_ref: &dyn State = state.borrow();
    let stroke_state = state_ref.downcast_ref::<StrokeState>();

    assert_eq!(stroke_state.unwrap().get_stroke_width(), 1f32);

    let fill_state = state_ref.downcast_ref::<FillState>();
    assert!(fill_state.is_none());
}
