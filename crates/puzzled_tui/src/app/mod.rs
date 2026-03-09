mod commands;
mod context;
mod events;

pub use commands::*;
pub use context::*;
pub use events::*;

use std::{collections::VecDeque, fmt::Debug, time::Duration};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use tokio::sync::{
    mpsc::{self, unbounded_channel},
    oneshot,
};

use crate::StatefulScreen;

const POLL_DURATION: Duration = Duration::from_millis(30);
const TICK_DURATION: Duration = Duration::from_millis(200);

pub struct App<M, A, T> {
    context: AppContext<T>,

    events_rx: mpsc::UnboundedReceiver<AppEvent>,

    engine: EventEngine<M, A>,

    shutdown: Option<oneshot::Sender<()>>,
}

impl<M, A, T> App<M, A, T>
where
    A: Clone + Send + ActionBehavior + 'static + Debug,
    M: Clone + Send + 'static,
{
    pub fn new(context: AppContext<T>, actions: EventTrie<M, A>) -> Self {
        let (events_tx, events_rx) = unbounded_channel();
        let events_tx2 = events_tx.clone();
        let (shutdown_tx, mut shutdown_rx) = oneshot::channel();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = &mut shutdown_rx => {
                        break;
                    }

                    _ = tokio::time::sleep(POLL_DURATION) => {
                        if let Ok(poll) = event::poll(Duration::ZERO) && poll &&
                            let Ok(event) = event::read()
                        {
                            let _ = events_tx2.send(AppEvent::new(event));
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

    pub async fn run(
        &mut self,
        init_screen: Box<dyn StatefulScreen<M, A, T>>,
    ) -> std::io::Result<()> {
        let mut terminal = ratatui::init();
        execute!(std::io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;

        // Set up screen management and enter the initial screen
        let mut screens: VecDeque<Box<dyn StatefulScreen<M, A, T>>> = VecDeque::from([init_screen]);

        screens
            .back_mut()
            .expect("Added initial screen")
            .on_enter(&mut self.context);

        // Set up an action resolver
        let (actions_tx, mut actions_rx) = mpsc::unbounded_channel();
        let resolver = ActionResolver::<M, A, T>::new(actions_tx);

        let mut render = true;

        loop {
            let screen = screens
                .back_mut()
                .expect("Should have a screen on the screen stack");

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
                    tracing::info!("Received event: {event}");

                    if let Some(command) = self.engine.push(event, &mut self.context.mode) {
                        resolver.fire_command(command);
                        render = true;
                    }
                }

                // Handle app event time out
                _ = tokio::time::sleep(TICK_DURATION) => {
                    if let Some(command) = self.engine.tick() {
                        resolver.fire_command(command);
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
                        _ => {}
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
