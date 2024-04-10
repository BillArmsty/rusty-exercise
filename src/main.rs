use rusty_exercise::{ settings::Settings, startup::Application };

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = Settings::new().expect("Failed to load settings.");
    let application = Application::build(settings).await?;
    application.run_until_stopped().await?;

    Ok(())
}
