mod app;
mod ui;


use std::{io, thread, time::Duration, error::Error};


fn main() -> Result<(), Box<dyn Error>> {
    ui::run()?;
    Ok(())
}

