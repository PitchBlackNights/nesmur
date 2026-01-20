use eframe::egui;
use nesmur::{PERSISTENT_DATA_PATH, app::App, cli::Cli, prelude::*, setup};
use std::path::PathBuf;

fn main() -> eframe::Result<()> {
    let _cli: Cli = setup::initial_setup();
    info!("Starting Nesmur...");

    let options: eframe::NativeOptions = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([550.0, 567.0]),
        persistence_path: Some(PathBuf::from(PERSISTENT_DATA_PATH)),
        ..Default::default()
    };
    let ret: Result<(), eframe::Error> = eframe::run_native(
        "nesmur",
        options,
        Box::new(|cc: &eframe::CreationContext<'_>| Ok(Box::new(App::new(cc)))),
    );

    info!("Stopped Nesmur");
    ret
}
