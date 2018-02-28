use std::collections::HashMap;

use iron::prelude::*;
use iron::Handler;
use iron::status;
use iron::mime::Mime;
use iron::mime::TopLevel::Application;
use iron::mime::SubLevel::Json;

use super::config::MERGED_FILE_PATH;
use super::file_io::read_file;
use super::json::{to_string, from_str};
use super::json::Merged;

struct Router {
    routes: HashMap<String, Box<Handler>>
}

impl Router {
    fn new() -> Self {
        Router { routes: HashMap::new() }
    }

    fn add_route<H>(&mut self, path: &str, handler: H) where H: Handler {
        self.routes.insert(path.to_string(), Box::new(handler));
    }
}

impl Handler for Router {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        match self.routes.get(&req.url.path().join("/")) {
            Some(handler) => handler.handle(req),
            None => Ok(Response::with(status::NotFound)),
        }
    }
}

fn content_type_json() -> Mime {
    Mime(Application, Json, vec![])
}

fn get_merged() -> Merged {
    let data = read_file(MERGED_FILE_PATH).unwrap();
    from_str(&data).unwrap()
}

pub fn http_server(url: &str) {
    let mut router = Router::new();

    router.add_route("", |_: &mut Request| {
        Ok(Response::with((status::Ok, "access to /merged")))
    });

    router.add_route("merged", |_: &mut Request| {
        println!("here?");
        let Merged(json) = get_merged();
        Ok(Response::with((
            content_type_json(),
            status::Ok,
            to_string(&json).unwrap()
        )))
    });

    Iron::new(router).http(url).unwrap();
}
