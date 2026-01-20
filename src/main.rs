use eframe::egui;
use log::info;
use nesmur::{app::App, cli::Cli, setup};

fn main() -> eframe::Result<()> {
    let _cli: Cli = setup::initial_setup();
    info!("Starting Nesmur...");

    let options: eframe::NativeOptions = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([550.0, 567.0]),
        ..Default::default()
    };
    let ret = eframe::run_native("nesmur", options, Box::new(|cc| Ok(Box::new(App::new(cc)))));

    info!("Stopped Nesmur");
    ret
}
