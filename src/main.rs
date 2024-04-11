use rusty_exercise::{ Config, startup::Application };

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::new().expect("Failed to load config.");
    
    let application = Application::build(config).await?;
    application.run_until_stopped().await?;

    Ok(())
}
