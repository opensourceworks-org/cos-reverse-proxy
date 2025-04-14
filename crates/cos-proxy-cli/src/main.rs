use dotenv::dotenv;
use cos_proxy_core::run_server;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    run_server();
    Ok(())
}