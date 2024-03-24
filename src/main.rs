use std::sync::Arc;

use anyhow::Result;
use zero2prod::config::Config;

use zero2prod::app::Application;
use zero2prod::email::EmailServiceImpl;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config = Config::new();
    let addr = format!("[::]:{}", config.port);

    let email_service = Arc::new(EmailServiceImpl::new(config.smtp_config.clone()));

    let app = Application::build(&config, addr, email_service).await?;
    app.run_until_stopped().await?;

    Ok(())
}
