use http::{httprequest::HttpRequest, httpresponse::HttpResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
// unused
// use std::default;
// use std::sync::Arc;

pub trait Handler {
  fn handle(req: &HttpRequest) -> HttpResponse;
  fn load_file(file_name: &str) -> Option<String> {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let full_path = format!("{}/{}", public_path, file_name);

      let contents = fs::read_to_string(full_path);
      contents.ok()
  } 
}

#[derive(Serialize, Deserialize)]
pub struct Status {
  id: i32,
  date: String,
  status: String,
}

pub struct StaticPageHandler;
pub struct PageNotFoundHandler;

impl Handler for PageNotFoundHandler {
  fn handle(_req: &HttpRequest) -> HttpResponse {
    HttpResponse::new("404", None, Self::load_file("404.html"))
  }
}

impl Handler for StaticPageHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
      let http::httprequest::Resource::Path(s) = &req.resource;
        let route: Vec<&str> = s.split("/").collect();
        match route[1] {
          "" => HttpResponse::new("200", None, Self::load_file("index.html")),
          "health" => HttpResponse::new("200", None, Self::load_file("index.html")),
          path => match Self::load_file(path) {
            Some(contents) => {
              let mut map: HashMap<&str, &str> = HashMap::new();
              if path.ends_with(".css") {
                map.insert("Content-Type:", "text/css");
              } else if path.ends_with(".js") {
                map.insert("Content-Type", "text/javascript");
              } else {
                  map.insert("Content-Type", "text/html");
              }
              HttpResponse::new("200", Some(map), 
            Some(contents))
            }
            None => HttpResponse::new("404", None, Self::load_file("404.html")),
          },
        }
    }
}
