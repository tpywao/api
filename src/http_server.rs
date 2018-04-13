use std::collections::HashMap;

use iron::{
    prelude::*,
    Handler,
    status,
    mime::{
        Mime,
        TopLevel::Application,
        SubLevel::Json,
    },
};

use json::to_string;
use memory_cache::{
    OriginCache,
    MergedCache,
};

struct Router {
    routes: HashMap<String, Box<Handler>>,
}

impl Router {
    fn new() -> Self {
        Router { routes: HashMap::new() }
    }

    fn add_route<H>(&mut self, path: &str, handler: H) where H: Handler {
        self.routes.insert(path.to_owned(), Box::new(handler));
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

pub fn http_server(url: String, origin_cache: OriginCache, merged_cache: MergedCache) {
    let mut router = Router::new();

    router.add_route("", |_: &mut Request| {
        Ok(Response::with((status::Ok, "access to /origin or /merged")))
    });

    router.add_route("origin", move |_: &mut Request| {
        let json = &*origin_cache.lock().unwrap();
        Ok(Response::with((
            content_type_json(),
            status::Ok,
            to_string(json).unwrap()
        )))
    });

    router.add_route("merged", move |_: &mut Request| {
        let json = &*merged_cache.lock().unwrap();
        Ok(Response::with((
            content_type_json(),
            status::Ok,
            to_string(json).unwrap()
        )))
    });

    Iron::new(router).http(url).unwrap();
}
