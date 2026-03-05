mod actions;
mod events;
mod screens;

pub use actions::*;
pub use events::*;
pub use screens::*;

use std::{collections::VecDeque, fmt::Debug, marker::PhantomData, time::Duration};

use crossterm::{
    event::{self, EnableMouseCapture},
    execute,
    terminal::EnterAlternateScreen,
};
use tokio::sync::mpsc::{self, unbounded_channel};

const POLL_DURATION: Duration = Duration::from_millis(30);
const TICK_DURATION: Duration = Duration::from_millis(200);

pub struct App<A, T> {
    actions: mpsc::UnboundedReceiver<Action<A>>,
    commands: CommandHistory<T>,
    state: T,

    _action: PhantomData<A>,
}

impl<A, T> App<A, T>
where
    A: Clone + Send + ActionHydrate + 'static,
{
    pub fn new(state: T, events: EventTrie<A>) -> Self {
        // Set up a channel to receive input events from the user
        let (actions_tx, actions_rx) = unbounded_channel();

        tokio::task::spawn_blocking(move || {
            let mut events = EventEngine::new(events, TICK_DURATION);
            let mut is_running = true;

            while is_running {
                // Poll for new events
                if let Ok(poll) = event::poll(POLL_DURATION)
                    && poll
                    && let Ok(event) = event::read()
                {
                    let app_event = AppEvent::new(event);

                    // See whether the application handles it and whether it needs action
                    if let Some(action) = events.push(app_event) {
                        if matches!(action, Action::Quit) {
                            is_running = false;
                        }

                        if actions_tx.send(action).is_err() {
                            break;
                        }
                    }
                }

                // Expire old events
                if let Some(action) = events.tick() {
                    if matches!(action, Action::Quit) {
                        is_running = false;
                    }

                    if actions_tx.send(action).is_err() {
                        break;
                    }
                }
            }
        });

        Self {
            state,
            commands: CommandHistory::default(),
            actions: actions_rx,
            _action: PhantomData,
        }
    }

    pub async fn run(&mut self, init_screen: Box<dyn StatefulScreen<A, T>>) -> std::io::Result<()> {
        let mut terminal = ratatui::init();
        execute!(std::io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;

        // Set up screen management and enter the initial screen
        let mut screens: VecDeque<Box<dyn StatefulScreen<A, T>>> = VecDeque::from([init_screen]);

        screens
            .back_mut()
            .expect("Added initial screen")
            .on_enter(&mut self.state);

        // Set up an action resolver
        let (actions_tx, mut actions_rx) = mpsc::unbounded_channel();
        let resolver = ActionResolver::<A, T>::new(actions_tx);

        loop {
            // Get the current screen and render its contents
            let screen = screens
                .back_mut()
                .expect("Should have a screen on the screen stack");

            terminal.draw(|frame| {
                screen.render(frame, &self.state);
            })?;

            // Then handle any actions or their results
            tokio::select! {
                // Handle incoming actions from the current screen
                Some(action) = self.actions.recv() => {
                    match action {
                        Action::Quit => break,
                        Action::Undo => self.commands.undo(&mut self.state),
                        Action::Redo => self.commands.redo(&mut self.state),

                        action => screen.on_action(action, resolver.clone(), &mut self.state)
                    }
                },

                // Resolve the result of completed actions
                Some(outcome) = actions_rx.recv() => match outcome {
                    // Completely exit the app
                    ActionOutcome::Exit => {
                        // Exit out of all screens in order
                        while let Some(mut screen) = screens.pop_back() {
                            screen.on_exit(&mut self.state);
                        }

                        break;
                    }

                    // Go to the previous screen
                    ActionOutcome::PreviousScreen => {
                        if screens.len() > 1 {
                            let mut old_screen = screens.pop_back().expect("Verified screens length");
                            old_screen.on_exit(&mut self.state);

                            let curr_screen = screens.back_mut().expect("Verified screen length");
                            curr_screen.on_resume(&mut self.state);
                        }
                    }

                    // Go to the next screen
                    ActionOutcome::NextScreen(mut next_screen) => {
                        // Pause the current screen
                        screen.on_pause(&mut self.state);

                        // Enter the next screen
                        next_screen.on_enter(&mut self.state);
                        screens.push_back(next_screen);
                    }
                    _ => {}
                }
            }
        }

        execute!(std::io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
        ratatui::restore();
        Ok(())
    }
}
