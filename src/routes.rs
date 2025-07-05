// route function calls 
use axum::{
    response::{
        Html
    },
    http::StatusCode
};
use handlebars::Handlebars;
use serde::Serialize;

#[derive(Serialize)]
struct Person { 
    name: String, 
    age: u32 
}

pub async fn four_o_four() -> Result<Html<String>, StatusCode> {
    let mut hbs = Handlebars::new();
    hbs.register_template_file("template", "path/to/template.hbs")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let person = Person { name: "Alice".to_string(), age: 30 };
    let rendered = hbs.render("template", &person)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Html(rendered))
}
