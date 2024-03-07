use crate::{
    config::Config,
    email::{EmailSender, EmailService},
};
use anyhow::Result;
use sqlx::postgres::PgPoolOptions;
use std::{marker::PhantomData, net::TcpListener};

use actix_web::{dev::Server, middleware::Logger, web, App, HttpServer};
use sqlx::{Pool, Postgres};

use crate::routes::{health_check, subscribe};

pub struct Application<'a, T> {
    port: u16,
    server: Server,
    _marker: PhantomData<&'a T>
}

impl<'a, T> Application<'a, T>
where
    T: EmailSender + Send + Sync + 'static,
{
    pub async fn build(
        config: &Config,
        addr: String,
        email_service: EmailService<'a, T>,
    ) -> Result<Self> {
        let pool = PgPoolOptions::new().connect(&config.db_config.url).await?;
        sqlx::migrate!().run(&pool).await?;

        let listener = TcpListener::bind(addr)?;
        let port = listener.local_addr().unwrap().port();
        let server = Self::run(listener, pool, email_service)?;

        Ok(Self { port, server, _marker: PhantomData })
    }

    fn run(
        listener: TcpListener,
        pool: Pool<Postgres>,
        email_service: EmailService<'a, T>,
    ) -> Result<Server> {
        let pool = web::Data::new(pool);
        let email_service = web::Data::new(email_service);
        let server = HttpServer::new(move || {
            let pool = pool.clone();
            let email_service = email_service.clone();

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

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}
