use bin::server_impl::server;

mod bin;
fn main() {
    let server = server::create(8080);
    server.start_listening();
}
