use std::path::Path;

use actix_cors::Cors;
use actix_service::Service;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer};
use paperclip::actix::{api_v2_operation, web, OpenApiExt};

use tracing::*;
use tracing_actix_web::TracingLogger;

use anyhow::{Context, Result};
use include_dir::{include_dir, Dir};

static DIST: Dir<'_> = include_dir!("frontend/dist/");

use crate::cli;

// Start REST API server with the desired address
pub async fn run() -> Result<()> {
    let server_address = &cli::CONFIGURATION.address;
    println!("\n\n\t\t Web GUI at:");
    println!("\t >>> http://{server_address} <<<\n\n");

    // Start HTTP server thread
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            // Add debug call for API access
            .wrap_fn(|req, srv| {
                trace!("{:#?}", &req);
                srv.call(req)
            })
            .wrap(TracingLogger::default())
            .wrap(actix_web::middleware::Logger::default())
            // Record services and routes for paperclip OpenAPI plugin for Actix.
            .wrap_api()
            .route(r"/{filename:.*}", web::get().to(root))
            .build()
    })
    .bind(server_address)
    .expect("Failed starting web API")
    .run()
    .await
    .map_err(anyhow::Error::msg)
}

#[api_v2_operation]
pub fn root(req: HttpRequest) -> HttpResponse {
    let mut filename = req.match_info().query("filename");
    if filename.is_empty() {
        filename = "index.html";
    }

    let extension = Path::new(&filename)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("");
    let mime = actix_files::file_extension_to_mime(extension).to_string();

    let file = match DIST.get_file(filename).context("Can't find file") {
        Ok(file) => file,
        Err(error) => {
            error!("Failed loading file {filename:?}: {error}");
            return HttpResponse::NotFound()
                .content_type("text/plain")
                .body("File does not exist");
        }
    };

    return match extension {
        "html" | "js" | "css" => {
            let body = match file.contents_utf8().context("Can't read file as utf-8") {
                Ok(body) => body,
                Err(error) => {
                    let msg = format!("Failed loading file {filename:?}: {error}");
                    error!(msg);
                    return HttpResponse::NotFound()
                        .content_type("text/plain")
                        .body(msg);
                }
            };
            HttpResponse::Ok().content_type(mime).body(body)
        }
        _ => {
            let body = file.contents();
            HttpResponse::Ok().content_type(mime).body(body)
        }
    };
}
