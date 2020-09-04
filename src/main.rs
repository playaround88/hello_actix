#[macro_use]
extern crate log;
extern crate stderrlog;

use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use hello_actix::sys::user::scoped_config as user;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("hello Rust world!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    stderrlog::new()
        .module(module_path!())
        .quiet(false)
        .verbosity(3)
        .init()
        .unwrap();
    info!("log init OK!");
    debug!("max log level debug!");

    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/static", ".").show_files_listing()) //static file
            .route("/", web::get().to(index))
            .configure(user)
    })
    .bind("127.0.0.1:8080")?
    .workers(2)
    .run()
    .await
}
