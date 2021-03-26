use actix_files as fs;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use package_manager::BinRepo;
use serde_yaml::from_reader;
use std::fs::File;
use std::time::SystemTime;

async fn with_param(req: HttpRequest, web::Path((name,)): web::Path<(String,)>) -> HttpResponse {
    let path = format!("./rootfs/var/www/{}/{}.db", &name, &name);
    let db_path = BinRepo::from(&path);

    HttpResponse::Ok().content_type("text/plain").body(format!(
        "{}",
        db_path
            .date
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/version/{name}").route(web::get().to(with_param)))
            .service(fs::Files::new("/", "./rootfs/var/www").show_files_listing())
    })
    .bind("127.0.0.1:3690")?
    .run()
    .await
}
