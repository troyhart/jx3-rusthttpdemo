use actix_files::NamedFile;
use actix_web::{web, App, HttpRequest, HttpServer, Responder, Result};
use std::path::PathBuf;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn index(req: HttpRequest) -> Result<NamedFile> {
    let path = match req.match_info().query("filename").parse::<PathBuf>() {
        Ok(path) if path.starts_with("static/") => path,
        _err => {
            println!("* Default for --> [{}: {:?}]", if _err.is_err() {"unspecified path"} else {"invalid path"}, _err.unwrap());
            PathBuf::from("static/index.html")
        },
    };
    println!("Path: {:?}", path);
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/f/{filename:.*}", web::get().to(index))
            .route("/{name}", web::get().to(greet))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
