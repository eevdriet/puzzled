use std::io;

use puzzled_tui::init_logging;

use crate::app::App;

mod app;

fn main() -> io::Result<()> {
    init_logging(true);
    println!("Hello, world!");

    let mut app = App::new();
    app.run()?;

    Ok(())
}
