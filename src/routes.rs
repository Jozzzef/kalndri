use hyper::service::{make_service_fn, service_fn};
use hyper::{body, Method, Request, Response, StatusCode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::Infallible;
use std::fs;
use http_body_util::Full;

// json user
#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

// Monolith function to handle all endpoints http requests
pub async fn handle_request(req: Request<body::Incoming>) -> Result<Response<Full<body::Bytes>>, Infallible> {

    // mach on endpoint
    let response = match (req.method(), req.uri().path()) {

        // WEB APP ENDPOINTS 🦊
        // (1) GET "/" 
        (&Method::GET, "/") => {
            // this needs to implement either serving a login page or a user's homepage
                // redirect to /login  
                // redirect to /home if there is a valid cookie for the user
        }
        // (1.1) GET "/login"
        (&Method::GET, "/login") => {
            serve_file("login.html").await
        }
        // (1.1.1) GET "/signup" -> the redirect is done through a ui link/button
        (&Method::GET, path) if path.starts_with("/user-") => {
            serve_file("signup.html").await
        }
        // (1.2) GET "/app" -> this is the core application
        (&Method::GET, "/app") => {
            serve_file("app.html").await

            // check if there is a valid cookie passed (just in case someone tries to come her
            // manually)

            //redirect to "/"
            //combinations: 
                // view: today,<n>_days,
        }

        (&Method::GET, path) if path.starts_with("/static/") => {
            serve_file(&path[1..]).await // Remove leading slash
        }

        // MUTATION API ENDPOINTS 🦣 
        // all updates to a user's data is done through APIs

        
        // DATA API ENDPOINTS 🐢
        // this is used for retrieving user data for the ui + custom user stats
        (&Method::GET, "/api/health") => {
            json_response(StatusCode::OK, &HashMap::from([("status", "healthy")]))
        }
        
        (&Method::GET, "/api/users") => {
            let users = vec![
                User { id: 1, name: "Alice".to_string(), email: "alice@example.com".to_string() },
                User { id: 2, name: "Bob".to_string(), email: "bob@example.com".to_string() },
            ];
            json_response(StatusCode::OK, &users)
        }
        
        (&Method::GET, path) if path.starts_with("/api/users/") => {
            let id = path.strip_prefix("/api/users/").unwrap();
            match id.parse::<u32>() {
                Ok(user_id) => {
                    let user = User {
                        id: user_id,
                        name: format!("User {}", user_id),
                        email: format!("user{}@example.com", user_id),
                    };
                    json_response(StatusCode::OK, &user)
                }
                Err(_) => error_response(StatusCode::BAD_REQUEST, "Invalid user ID"),
            }
        }
        
        (&Method::POST, "/api/users") => {
            // In a real app, you'd parse the request body here
            let new_user = User {
                id: 3,
                name: "New User".to_string(),
                email: "new@example.com".to_string(),
            };
            json_response(StatusCode::CREATED, &new_user)
        }
        
        (&Method::DELETE, path) if path.starts_with("/api/users/") => {
            let id = path.strip_prefix("/api/users/").unwrap();
            match id.parse::<u32>() {
                Ok(_) => {
                    let response = HashMap::from([("message", "User deleted successfully")]);
                    json_response(StatusCode::OK, &response)
                }
                Err(_) => error_response(StatusCode::BAD_REQUEST, "Invalid user ID"),
            }
        }
        
        // Catch-all for 404
        _ => error_response(StatusCode::NOT_FOUND, "Not Found"),
    };
    
    Ok(response)
}


async fn serve_file(file_path: &str) -> Response<Body> {
    match fs::read_to_string(file_path) {
        Ok(content) => {
            let content_type = match file_path.split('.').last() {
                Some("html") => "text/html",
                Some("css") => "text/css",
                Some("js") => "application/javascript",
                Some("json") => "application/json",
                _ => "text/plain",
            };
            
            Response::builder()
                .status(StatusCode::OK)
                .header("content-type", content_type)
                .body(Body::from(content))
                .unwrap()
        }
        Err(_) => error_response(StatusCode::NOT_FOUND, "File not found"),
    }
}

fn json_response<T: Serialize>(status: StatusCode, data: &T) -> Response<Body> {
    let json = serde_json::to_string(data).unwrap();
    Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(Body::from(json))
        .unwrap()
}

fn error_response(status: StatusCode, message: &str) -> Response<Body> {
    let error = HashMap::from([("error", message)]);
    json_response(status, &error)
}
