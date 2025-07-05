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
struct ErrorObject { 
    status_code: i16 
}

use std::env;

fn retrieve_htmx(relative_file_path: &str) -> Result<String, StatusCode> {

    let current_dir = match env::current_dir(){
        Ok(val) => {val},
        Err(error) => {
            eprintln!("Error: {}", error);
            return Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    };// This returns PathBuf
    let full_path = current_dir.join(relative_file_path); // PathBuf::join()
    println!("{:?}", &full_path);

    let mut hbs = Handlebars::new();
    match hbs.register_template_file("error_htmx", &full_path){
        Ok(value) => println!("Result: {:?}", value),
        Err(error) => {
            println!("{:?}", &full_path);
            eprintln!("Error: {}", error);
            return Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
    println!("{:?}", &hbs);
    let err_obj = ErrorObject { status_code: 404 };
    match hbs.render("error_htmx", &err_obj){
        Ok(value) => {
            println!("Result: {:?}", value);
            Ok(value)
        },
        Err(error) => {
            eprintln!("Error: {}", error);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn four_o_four() -> Result<Html<String>, StatusCode> {
    Ok(Html(retrieve_htmx("src/templates/error.hbs")?))
    //Ok(Html("<div>wtf</div>".to_string()))
}
