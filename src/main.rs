use std::sync::Arc;

use zero2prod::config::Config;

use zero2prod::app::Application;
use zero2prod::email::EmailServiceImpl;

#[tokio::main]
async fn main() -> Result<(), String> {
    tracing_subscriber::fmt::init();

    let config = Config::new();
    let addr = format!("[::]:{}", config.port);

    let email_service = Arc::new(EmailServiceImpl::new(config.smtp_config.clone()));

    let app = Application::build(&config, addr, email_service).await?;
    app.run_until_stopped()
        .await
        .map_err(|e| format!("Error running application {}", e))?;

    Ok(())
}
