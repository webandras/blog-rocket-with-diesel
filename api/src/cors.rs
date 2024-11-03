use rocket::http::{Header, Method, Status};
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        if _request.method() == Method::Options {
            response.set_status(Status::NoContent);
            response.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "POST, PUT, PATCH, GET, DELETE",
            ));
            response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        }

        /*
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        */

        let host = _request.host().unwrap();
        if host.domain() == "127.0.0.1" {
            response.set_header(Header::new("Access-Control-Allow-Origin", "http://localhost:5173"));
        } else {
            response.set_header(Header::new("Access-Control-Allow-Origin", "https://rust-blog.webandras.hu"));
        }

        // response.set_header(Header::new("Access-Control-Allow-Methods", "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET"));
        // response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        response.set_header(Header::new("Vary", "Origin"));
    }
}