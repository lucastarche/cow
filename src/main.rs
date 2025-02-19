use app::CowApp;

mod app;
mod node_graph;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let cow_app = CowApp::new().await?;
    if let Err(e) = eframe::run_native(
        "COW",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(cow_app))),
    ) {
        println!("{e:#?}");
    }

    Ok(())
}
