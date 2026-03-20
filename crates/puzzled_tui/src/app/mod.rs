mod commands;
mod context;
mod events;
mod types;

pub use commands::*;
pub use context::*;
pub use events::*;
pub use types::*;

use std::{collections::VecDeque, time::Duration};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use tokio::sync::{
    mpsc::{self, unbounded_channel},
    oneshot,
};

use crate::Screen;

const POLL_DURATION: Duration = Duration::from_millis(5);
const TICK_DURATION: Duration = Duration::from_millis(200);

pub struct App<A, T, M, S> {
    context: AppContext<A, T, M, S>,

    events_rx: mpsc::UnboundedReceiver<AppEvent>,

    engine: EventEngine<A, T, M>,

    shutdown: Option<oneshot::Sender<()>>,
}

impl<A, T, M, S> App<A, T, M, S>
where
    A: ActionBehavior + 'static,
    T: TextObjectBehavior + 'static,
    M: MotionBehavior + 'static,
{
    pub fn new(context: AppContext<A, T, M, S>, actions: EventTrie<A, T, M>) -> Self {
        let (events_tx, events_rx) = unbounded_channel();
        let (shutdown_tx, mut shutdown_rx) = oneshot::channel();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = &mut shutdown_rx => {
                        break;
                    }

                    _ = tokio::time::sleep(POLL_DURATION) => {
                        while let Ok(true) = event::poll(Duration::ZERO) &&
                            let Ok(event) = event::read()
                        {
                            let _ = events_tx.send(AppEvent::new(event));
                        }
                    }
                }
            }
        });

        let engine = EventEngine::new(actions, TICK_DURATION);

        Self {
            context,
            engine,

            events_rx,
            shutdown: Some(shutdown_tx),
        }
    }

    pub async fn run(&mut self, init_screen: Box<dyn Screen<A, T, M, S>>) -> std::io::Result<()> {
        let mut terminal = ratatui::init();
        execute!(std::io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;

        // Set up screen management and enter the initial screen
        let mut screens: VecDeque<Box<dyn Screen<A, T, M, S>>> = VecDeque::from([init_screen]);

        screens
            .back_mut()
            .expect("Added initial screen")
            .on_enter(&mut self.context);

        // Set up an action resolver
        let (actions_tx, mut actions_rx) = mpsc::unbounded_channel();
        let resolver = AppResolver::<A, T, M, S>::new(actions_tx);

        let mut render = true;

        loop {
            let screen = screens
                .back_mut()
                .expect("Should have a screen on the screen stack");
            let override_mode = screen.override_mode();

            // Get the current screen and render its contents
            if render {
                terminal.draw(|frame| {
                    screen.render(frame.area(), frame.buffer_mut(), &mut self.context);
                })?;

                render = false;
            }

            // Then handle any actions or their results
            tokio::select! {
                // Handle raw app events
                Some(event) = self.events_rx.recv() => {
                    if event.is_resize() {
                        render = true;
                    }

                    for effect in self.engine.push(event, override_mode).effects {
                        match effect {
                            EventEffect::Command(command) => {
                                resolver.fire_command(command);
                            }
                            EventEffect::Mode(mode) => {
                                resolver.set_mode(mode);
                            }
                        }
                    }
                }

                // Handle app event time out
                _ = tokio::time::sleep(TICK_DURATION) => {
                    for effect in self.engine.tick(override_mode).effects {
                        match effect {
                            EventEffect::Command(command) => {
                                resolver.fire_command(command);
                            }
                            EventEffect::Mode(mode) => {
                                resolver.set_mode(mode);
                            }
                        }
                    }

                    if screen.on_tick(&self.context) {
                        render = true;
                    }
                }

                // Resolve the result of completed actions
                Some(outcome) = actions_rx.recv() => {
                    match outcome {
                        // Handle actions
                        CommandOutcome::Command(command) => {
                            screen.on_command(command, resolver.clone(), &mut self.context);
                        }

                        CommandOutcome::Mode(mode) => {
                            self.engine.set_mode(mode);
                            screen.on_mode(mode, resolver.clone(), &mut self.context);
                        }

                        // Completely exit the app
                        CommandOutcome::Quit => {
                            break;
                        }

                        // Go to the previous screen
                        CommandOutcome::PreviousScreen => {
                            if screens.len() > 1 {
                                let mut old_screen = screens.pop_back().expect("Verified screens length");
                                old_screen.on_exit(&mut self.context);

                                let curr_screen = screens.back_mut().expect("Verified screen length");
                                curr_screen.on_resume(&mut self.context);
                            }
                        }

                        // Go to the next screen
                        CommandOutcome::NextScreen(mut next_screen) => {
                            // Pause the current screen
                            screen.on_pause(&mut self.context);

                            // Enter the next screen
                            next_screen.on_enter(&mut self.context);
                            screens.push_back(next_screen);
                        }

                        // Render a popup over the current screen
                        CommandOutcome::OpenPopup => {
                            screen.on_pause(&mut self.context);
                            screen.on_popup_open(&mut self.context);
                        }

                        // Close the rendered popup
                        CommandOutcome::ClosePopup => {
                            screen.on_popup_close(&mut self.context);
                            screen.on_resume(&mut self.context);
                        }
                    }

                    // Re-render to communicate the state change
                    render = true;
                }
            }
        }

        // Shutdown event loop
        if let Some(tx) = self.shutdown.take() {
            let _ = tx.send(());
            eprintln!("Shutdown");
        }

        // Exit out of all screens in order
        while let Some(mut screen) = screens.pop_back() {
            screen.on_exit(&mut self.context);
        }

        execute!(std::io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
        ratatui::restore();
        Ok(())
    }
}
