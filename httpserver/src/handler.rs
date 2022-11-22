use std::collections::HashMap;
use std::env;
use std::fs;
use http::*;
use http::httprequest::{HttpRequest, Resource};
use http::httpresponse::HttpResponse;
use serde::{Deserialize, Serialize};
use serde_json::Result;




pub trait Handler {
    fn handle(req: &HttpRequest) -> HttpResponse;

    fn load_file(file_name: &str) -> Option<String> {
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", public_path, file_name);

        let content = fs::read_to_string(full_path);
        content.ok()
    }
}

pub struct StaticHandler;
pub struct PageNotFoundHandler;
pub struct WebServiceHandler;


impl Handler for StaticHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        match &req.resource { Resource::Path(path) => {
            println!("path={}", &path);
            let mut map: HashMap<&str, &str> = HashMap::new();
            map.insert("Content-Type", "text/html");
            let response = HttpResponse::new("200", Some(map), Self::load_file("404.html"));
            response

        } }
    }
}


impl Handler for PageNotFoundHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let mut map: HashMap<&str, &str> = HashMap::new();
        map.insert("Content-Type", "text/html");
        HttpResponse::new("200",Some(map), Self::load_file("404.html"))
    }
}

#[derive(Serialize, Deserialize)]
pub struct Order {
    id: i32,
    name: String,
}
impl Handler for WebServiceHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let mut map: HashMap<&str, &str> = HashMap::new();
        map.insert("Content-Type", "application/json");
        let json = r#"
        [{"id":1,
         "name":"hu"
        },
        {"id":2,
         "name":"xxx"
        }]
        "#;
        let result: Vec<Order> = serde_json::from_str(json).unwrap();
        let option = serde_json::to_string(&result).ok();
        HttpResponse::new("200", Some(map), Some(json.to_string()))
    }
}



