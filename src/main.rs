use bessie::Bessie;
use eframe::NativeOptions;

mod barn;
mod bessie;
mod farmer_john;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    if let Err(e) = eframe::run_native(
        "COW",
        NativeOptions::default(),
        Box::new(|_| Ok(Box::new(Bessie::new()))),
    ) {
        println!("Error: {e:#?}");
    }

    Ok(())
}
