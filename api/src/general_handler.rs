use rocket::{catch, options, Request};
use rocket::http::Status;
use shared::response_models::{Response, ResponseBody};

/// Catches all OPTION requests in order to get the CORS related Fairing triggered.
#[options("/<_..>")]
pub fn all_options_handler() {
    /* Intentionally left empty */
}

#[catch(400)]
pub fn bad_request(req: &Request) -> String {
    let response = Response { body: ResponseBody::Error(format!("400 Bad Request - {}", req.uri())) };
    serde_json::to_string(&response).unwrap()
}

#[catch(401)]
pub fn unauthorized(req: &Request) -> String {
    let response = Response { body: ResponseBody::Error(format!("401 Unauthorized -  {}", req.uri())) };
    serde_json::to_string(&response).unwrap()
}

#[catch(403)]
pub fn forbidden(req: &Request) -> String {
    let response = Response { body: ResponseBody::Error(format!("403 Forbidden - {}", req.uri())) };
    serde_json::to_string(&response).unwrap()
}

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    let response = Response { body: ResponseBody::Error(format!("404 Not Found - {}", req.uri())) };
    serde_json::to_string(&response).unwrap()
}

#[catch(429)]
pub fn too_many_redirects(req: &Request) -> String {
    let response = Response { body: ResponseBody::Error(format!("429 Too Many Redirects - {}", req.uri())) };
    serde_json::to_string(&response).unwrap()
}

#[catch(500)]
pub fn internal_server_error(req: &Request) -> String {
    let response = Response { body: ResponseBody::Error(format!("500 Internal Server Error - {}", req.uri())) };
    serde_json::to_string(&response).unwrap()
}

#[catch(502)]
pub fn bad_gateway(req: &Request) -> String {
    let response = Response { body: ResponseBody::Error(format!("502 Bad Gateway -  {}", req.uri())) };
    serde_json::to_string(&response).unwrap()
}

#[catch(default)]
pub fn default(status: Status, req: &Request) -> String {
    let response = Response { body: ResponseBody::Error(format!("{} ({})", status, req. uri())) };
    serde_json::to_string(&response).unwrap()
}
