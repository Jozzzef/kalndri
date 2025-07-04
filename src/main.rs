mod routes;
use routes::four_o_four;
use axum::{
    routing::get,
    Router
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(four_o_four));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
