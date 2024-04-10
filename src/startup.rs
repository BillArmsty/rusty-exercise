use actix_session::{ storage::RedisSessionStore, SessionMiddleware };
use actix_web::{ cookie::{ Key, SameSite }, dev::Server, web, App, HttpServer };

use tracing_actix_web::TracingLogger;
use secrecy::{ ExposeSecret, Secret };
use diesel::pg::{ PgConnection, PgConnectionOptions };

use crate::{ api::{ login, signup }, settings::{ Settings, DatabaseConfig } };

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(config: Settings) -> Result<Self, std::io::Error> {
        let pool = get_connection_pool(&config.database)?;

        let server = run(config.server.port, pool)?.await.expect("Server failed to start");

        Ok(Self {
            port: config.server.port,
            server,
        })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn get_connection_pool(
    config: &DatabaseConfig
) -> Result<PgConnection, diesel::ConnectionError> {
    let options = PgConnectionOptions::new().connect(&config.url)?;
    Ok(PgConnection::establish(&options)?)
}

pub async fn run(conn: PgConnection, port: u16) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            // enable logger
            .wrap(TracingLogger::default())
            .wrap(
                SessionMiddleware::builder(store.clone(), secret_key.clone())
                    // allow the cookie to be accessed from JavaScript
                    .cookie_http_only(false)
                    // allow the cookie only from  the current domain
                    .cookie_same_site(SameSite::Strict)
                    .build()
            )
            .route("/signup", web::post().to(signup))
            .route("/login", web::post().to(login))
    }).run();
    Ok(server)
}
