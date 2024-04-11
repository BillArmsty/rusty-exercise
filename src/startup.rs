use std::net::TcpListener;

use actix_web::{ dev::Server, web, App, HttpServer };

use anyhow::Context;
use diesel::{ pg::PgConnection, r2d2::ConnectionManager };
use r2d2::Pool;

use crate::{ api::{ login, register }, Config, types::PgPool };

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(config: Config) -> Result<Self, anyhow::Error> {
        let pool = get_connection_pool(&config.database_url);

        let listener = TcpListener::bind(config.url())?;

        let port = listener.local_addr().context("Failed to get local port")?.port();

        let server = run(pool, listener).await?;

        Ok(Self {
            port,
            server,
        })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}

pub fn get_connection_pool(database_url: &str) -> PgPool {

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = Pool::builder().build(manager).expect("Failed to create connection pool.");

    pool
}

pub async fn run(pool: PgPool, listener: TcpListener) -> Result<Server, std::io::Error> {
    let pool = web::Data::new(pool);

    let server = HttpServer::new(move || {
        App::new()

            .route("/signup", web::post().to(register))
            .route("/login", web::post().to(login))
            .app_data(pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
