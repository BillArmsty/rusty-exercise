use rusty_exercise::{
    Config,
    startup::Application,
    // telemetry::{ get_subscriber, init_subscriber },
};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::new().expect("Failed to load config.");

    // let subscriber = get_subscriber("rusty_exercise", &config.log.level, std::io::stdout);
    // init_subscriber(subscriber);

    let application = Application::build(config).await?;
    application.run_until_stopped().await?;

    Ok(())
}
