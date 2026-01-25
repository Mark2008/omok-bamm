pub mod bot;
pub mod core;
pub mod ui;

use tracing_subscriber::prelude::*;
use tracing_subscriber::fmt;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::format::FmtSpan;

fn init_trace() {
    let file = std::fs::File::create("trace.log").unwrap();

    let filter = EnvFilter::new(
        "info,debug,eframe=warn,egui=warn,wgpu=warn,winit=warn"
    );

    let subscriber = tracing_subscriber::registry()
        .with(filter)
        .with(
            fmt::layer()
                .with_writer(file)
                .with_ansi(false)
                .with_span_events(FmtSpan::ENTER | FmtSpan::EXIT)
        );

    tracing::subscriber::set_global_default(subscriber).unwrap();
    
    tracing::debug!("aaaa");
}

// The main function where our program starts
fn main() -> Result<(), eframe::Error> {
    init_trace();
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "egui Demo",
        options,
        Box::new(|_cc| Ok(Box::new(ui::MyApp::default()))),
    )
}