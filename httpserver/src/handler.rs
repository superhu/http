use std::env;
use std::fs;
use http::*;
use http::httprequest::HttpRequest;
use http::httpresponse::HttpResponse;


pub trait Handler {
    fn handle(req: &HttpRequest) -> HttpResponse;

    fn load_file(file_name: &str) -> Option<String> {
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", public_path, file_name);

        let content = fs::read_to_string(file_name);
        content.ok()
    }
}

pub struct StaticHandler;
pub struct PageNotFoundHandler;
pub struct WebServiceHandler;



