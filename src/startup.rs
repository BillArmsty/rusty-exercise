use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use actix_web::{ cookie::{ Key, SameSite }, dev::Server, web, App, HttpServer };

use actix_session::{ storage::CookieSessionStore, SessionMiddleware };

use anyhow::Context;
use diesel::{ pg::PgConnection, r2d2::ConnectionManager };
use r2d2::Pool;

use crate::{ api::{ login, logout_user, register, get_users }, types::PgPool, Config };

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

    let secret_key = Key::generate();

    let server = HttpServer::new(move || {
        App::new()

            .wrap(TracingLogger::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_http_only(false)
                    .cookie_same_site(SameSite::Strict)
                    .cookie_secure(false)
                    .build()
            )

            .route("/signup", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/logout", web::get().to(logout_user))
            .route("/users", web::get().to(get_users))
            .app_data(pool.clone())
    })
        .listen(listener)?
        .run();
    Ok(server)
}
