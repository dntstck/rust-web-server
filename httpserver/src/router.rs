use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler};
use http::{httprequest, httprequest::HttpRequest, httpresponse::HttpResponse};
use std::io::prelude::*;

pub struct Router;
impl Router {
  pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
    match req.method {
      httprequest::Method::Get => match &req.resource {
        httprequest::Resource::Path(s) => {
          let route: Vec<&str> = s.split("/").collect();
          match route[1] {
            // if route matches /api invoke webservice. WS not implemented yet so we'll just direct it to 404 for now
            "api" => {
              let resp: HttpResponse = PageNotFoundHandler::handle(&req);
              let _ = resp.send_response(stream);
            }
            _ => {
              let resp: HttpResponse = StaticPageHandler::handle(&req);
              let _ = resp.send_response(stream);
            }
          }
        }
      },
      _ => {
        let resp: HttpResponse = PageNotFoundHandler::handle(&req);
        let _ = resp.send_response(stream);
      }
    }
  }
}