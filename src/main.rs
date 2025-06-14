use poem::{Route, Server, listener::TcpListener};

mod handlers;
mod image_proc;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "debug");
        }
    }

    tracing_subscriber::fmt::init();

    let app = Route::new()
        .at("/", poem::get(handlers::root::index))
        .at("/upload", poem::post(handlers::handle_image::upload));

    println!("Server running at http://127.0.0.1:3000");

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
