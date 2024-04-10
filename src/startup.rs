use actix_web::{ dev::Server, web, App, HttpServer };

use diesel::{ pg::PgConnection, Connection };

use crate::{ api::{login, register}, settings::{ Settings, DatabaseConfig } };

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(config: Settings) -> Result<Self, std::io::Error> {
        let pool = get_connection_pool(&config.database).expect("Failed to connect to database.");

        let server = run(pool, config.server.port).await?;

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
    let options = config.database_url.clone();
    Ok(PgConnection::establish(&options)?)
}


pub async fn run(conn: PgConnection, port: u16) -> Result<Server, std::io::Error> {
    let pool = web::Data::new(conn);
    let server = HttpServer::new(move || {
        App::new()

            .route("/signup", web::post().to(register))
            .route("/login", web::post().to(login))
            .app_data(pool.clone())
    }).run();
    Ok(server)
}
