use bin::{server_impl::server, structs::{rest_handler::RestHandler, routes::RoutesBuilder}};

mod bin;
fn main() {
    let mut routes = RoutesBuilder::new();
    routes
    .add_route("GET","/",Box::new(|_|{
        (httpstatus::StatusCode::Ok,"<h1>Hello, World!</h1>".to_string())
    }))
    .add_route("POST", "/", Box::new(|_|{
        (httpstatus::StatusCode::Ok,"<h1>Hello, POST!</h1>".to_string())
    }));
    let server = server::create(Some(routes));
    server.start_listening();
}
