//! ai-terminal — Sorabbyngo OS AI-native terminal emulator

mod terminal;
mod shell;
mod nlp;
mod completion;
mod safety;
mod voice;
mod ui;

use anyhow::Result;
use tracing::info;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    info!("ai-terminal starting");
    Ok(())
}
