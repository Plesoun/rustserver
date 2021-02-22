use super::server::Handler;
use super::http::{StatusCode, Request, Response};

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
	fn handle_request(&mut self, request: &Request) -> Response {
		Response::new(StatusCode::Ok, Some("<h1> Phewphew!! </h1>".to_string()))
	}
}
