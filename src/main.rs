use anyhow::Result;
use zero2prod::config::Config;

use zero2prod::app::Application;
use zero2prod::email::{EmailService, LettreEmailSender};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config = Config::new();
    let addr = format!("[::]:{}", config.port);

    static EMAIL_SENDER: LettreEmailSender = LettreEmailSender {};

    let email_service = EmailService::new(config.smtp_config.clone(), &EMAIL_SENDER);

    let app = Application::build(&config, addr, email_service).await?;
    app.run_until_stopped().await?;

    Ok(())
}
