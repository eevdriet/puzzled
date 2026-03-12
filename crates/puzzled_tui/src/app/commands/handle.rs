use crate::{ActionResolver, AppContext, Command, EventMode, StatefulScreen};

pub enum CommandOutcome<M, A, T> {
    // Handled externally
    Command(Command<M, A>),
    Mode(EventMode),

    // Screen management
    Quit,
    PreviousScreen,
    NextScreen(Box<dyn StatefulScreen<M, A, T>>),
}

pub trait HandleCommand<M, A, T> {
    type State;

    fn handle_command(
        &mut self,
        _command: Command<M, A>,
        _resolver: ActionResolver<M, A, T>,
        _ctx: &mut AppContext<T>,
        _state: &mut Self::State,
    ) -> bool;
}

// pub fn handle_action<H, M, A, T, S>(
//     handler: &mut H,
//     command: &Command<M, A>,
//     resolver: ActionResolver<M, A, T>,
//     ctx: &mut AppContext<T>,
//     state: &mut S,
// ) -> bool
// where
//     H: HandleCustomAction<M, A, T, S>,
// {
//     match (command.base_action(), command.custom_action()) {
//         (Some(action), None) => handler.handle_base_action(action, resolver, ctx),
//         (None, Some(action)) => handler.handle_custom_action(action, resolver, ctx, state),
//         (None, None) => false,
//         _ => unreachable!("Either a base, custom or no action should be set"),
//     }
// }

// pub fn handle_pure_motion<H, M, A, T, MS, OS, P>(
//     handler: &mut H,
//     cursor: &mut P,
//     command: &Command<M, A>,
//     motion_state: &mut MS,
// ) -> bool
// where
//     H: CustomMotionRange<M, MS, Position = P> + ApplyOperator<Position = P, State = OS>,
//     P: Clone,
// {
//     // Find the motion range and move the cursor to the last (valid) position
//     let start = cursor.clone();
//     let positions = motion_range(
//         handler,
//         start,
//         command.count(),
//         command.motion(),
//         motion_state,
//     );
//
//     let Some(end) = positions.last() else {
//         return false;
//     };
//
//     *cursor = end.clone();
//     true
// }
//
// pub fn handle_motion_with_operator<H, M, A, T, MS, OS, P>(
//     handler: &mut H,
//     cursor: &mut P,
//     command: &Command<M, A>,
//     motion_state: &mut MS,
//     operator_state: &mut OS,
// ) -> bool
// where
//     H: CustomMotionRange<M, MS, Position = P> + ApplyOperator<Position = P, State = OS>,
//     P: Clone,
// {
//     // Make sure an operator is defined
//     let Some(op) = command.operator() else {
//         return false;
//     };
//
//     // Find the motion range and move the cursor to the last (valid) position
//     let start = cursor.clone();
//     let positions = motion_range(
//         handler,
//         start,
//         command.count(),
//         command.motion(),
//         motion_state,
//     );
//
//     let Some(end) = positions.last() else {
//         return false;
//     };
//
//     *cursor = end.clone();
//
//     // Apply the operator to all positions
//
//     true
// }
//
// pub fn handle_text_object<H, M, A, T, MS, OS, P>(
//     handler: &mut H,
//     cursor: &mut P,
//     command: &Command<M, A>,
//     motion_state: &mut MS,
//     operator_state: &mut OS,
// ) -> bool
// where
//     H: CustomMotionRange<M, MS, Position = P> + ApplyOperator<Position = P, State = OS>,
//     P: Clone,
// {
//     // Find the motion range and move the cursor to the last (valid) position
//     let start = cursor.clone();
//     let positions = motion_range(
//         handler,
//         start,
//         command.count(),
//         command.motion(),
//         motion_state,
//     );
//
//     let Some(end) = positions.last() else {
//         return false;
//     };
//
//     *cursor = end.clone();
//
//     // If some operator was defined, apply it to all positions
//     if let Some(op) = command.operator() {
//         handler.apply_operator(op, positions, operator_state);
//     }
//
//     true
// }
//
// fn motion_range<H, M, S>(
//     handler: &mut H,
//     start: H::Position,
//     count: usize,
//     motion: &Motion<M>,
//     state: &mut S,
// ) -> Vec<H::Position>
// where
//     H: CustomMotionRange<M, S>,
// {
//     match motion {
//         Motion::Other(custom) => handler
//             .custom_motion_range(start, count, custom, state)
//             .into_iter()
//             .collect(),
//         motion => handler
//             .base_motion_range(start, count, motion)
//             .into_iter()
//             .collect(),
//     }
// }
