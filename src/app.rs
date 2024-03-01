use sqlx::postgres::PgPoolOptions;
use crate::{config::Config, subscribe};
use std::net::TcpListener;
use anyhow::Result;

use actix_web::{dev::Server, middleware::Logger, web, App, HttpServer};
use sqlx::{Pool, Postgres};

use crate::health_check;

pub struct Application {
    pub port: u16,
    pub server: Server
}

impl Application {
    pub async fn build(config: &Config, addr: String) -> Result<Self> {
        let pool = PgPoolOptions::new().connect(&config.db_config.url).await?;
        sqlx::migrate!().run(&pool).await?;

        let listener = TcpListener::bind(addr)?;
        let port = listener.local_addr().unwrap().port();
        let server = Self::run(listener, pool)?;

        Ok(Self {
            port,
            server
        })
    }

    pub fn run(
        listener: TcpListener,
        pool: Pool<Postgres>,
    ) -> Result<Server> {
        let pool = web::Data::new(pool);
        let server = HttpServer::new(move || {
            let pool = pool.clone();
    
            App::new()
                .wrap(Logger::default())
                .route("/health_check", web::get().to(health_check))
                .route("/subscriptions", web::post().to(subscribe))
                .app_data(pool)
        })
        .listen(listener)?
        .run();
        Ok(server)
    }
}