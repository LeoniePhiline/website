// <directory>
use ntex::web;
use ntex_files as fs;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| {
        web::App::new().service(fs::Files::new("/static", ".").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
// </directory>
