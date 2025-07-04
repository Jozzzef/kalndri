// route function calls 

use axum::{
    routing::get,
    Router,
    response::{
        Html
    }
};

pub async fn four_o_four() -> Html<&'static str> {
    Html("<div>Sorry, page not found (404!)</div>")
}
