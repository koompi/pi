use actix_files as fs;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use pi::{
    statics::{PUB_DIR, SERVER_CFG_DIR, SERVER_CFG_FILE},
    utils::prepare_bases,
    BinRepo,
};
use serde::{Deserialize, Serialize};
use serde_yaml::from_reader;
use std::{fs::File, path::PathBuf, time::SystemTime};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ServerConfig {
    pub repo_root: String,
}

impl ServerConfig {
    pub fn new() -> Self {
        Self {
            repo_root: PUB_DIR.to_str().unwrap().to_string(),
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    prepare_bases(vec![SERVER_CFG_DIR.to_path_buf(), PUB_DIR.to_path_buf()]).unwrap();

    if !SERVER_CFG_FILE.as_path().exists() {
        let mut file = File::create(SERVER_CFG_FILE.as_path()).unwrap();
        serde_yaml::to_writer(&mut file, &ServerConfig::new()).unwrap()
    }

    HttpServer::new(|| {
        let cfg = cfg_data();

        App::new()
            .service(web::resource("/version/{name}").route(web::get().to(with_param)))
            .service(fs::Files::new("/", &cfg.repo_root).show_files_listing())
    })
    .bind("127.0.0.1:3690")?
    .run()
    .await
}

fn cfg_data() -> ServerConfig {
    let cfg_file = File::open(SERVER_CFG_FILE.as_path()).unwrap();
    let cfg: ServerConfig = serde_yaml::from_reader(cfg_file).unwrap();
    cfg
}

async fn with_param(req: HttpRequest, web::Path((name,)): web::Path<(String,)>) -> HttpResponse {
    let cfg = cfg_data();
    let db_dir = PathBuf::from(&cfg.repo_root);
    let db_path = db_dir.join(&format!("{}/{}.db", &name, &name));
    let db_data = BinRepo::from(&db_path.to_str().unwrap());

    HttpResponse::Ok().content_type("text/plain").body(format!(
        "{}",
        db_data
            .date
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    ))
}
