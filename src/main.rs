#[allow(dead_code)]
#[allow(unused_variables)]
//use std::env::args;
//use clap;
use todo::task::*;



fn main() -> eframe::Result<()> {
    let app = todo::task::App::default();
    let options = eframe::NativeOptions{
        persist_window: true,
        ..Default::default()
    };

    eframe::run_native(
        "Todo App",
        options,
        Box::new(|_cc| Ok(Box::new(app))),
    )
}

