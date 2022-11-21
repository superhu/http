use std::io::Write;
use http::httprequest;
use http::httprequest::HttpRequest;
use http::httpresponse::HttpResponse;
use crate::handler::{Handler, PageNotFoundHandler, StaticHandler, WebServiceHandler};


pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        match req.method {
            httprequest::Method::Get => match &req.resource {
                httprequest::Resource::Path(s) => {
                    let route: Vec<&str> = s.split("/").collect();
                    match route[1] {
                        "api" => {
                            let resp: HttpResponse = WebServiceHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                        _ => {
                            let resp: HttpResponse = StaticHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }

                    }
                }
            }
            _ => {
                let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
}