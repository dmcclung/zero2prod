use zero2prod::config::Config;
use anyhow::Result;

use zero2prod::app::Application;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config = Config::new();
    let addr = format!("[::]:{}", config.port);

    let app = Application::build(&config, addr).await?;

    app.server.await?;
    
    Ok(())
}
