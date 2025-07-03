mod routes;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    routes::handle_request(req);
}
