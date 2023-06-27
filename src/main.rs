use ::std::env;
use server::Server;
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

fn main() {
    let port = 3000;
    
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let server = Server::new(format!("127.0.0.1:{}", port));
    server.run(WebsiteHandler::new(public_path));
}
