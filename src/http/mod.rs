pub mod row_methods;
pub mod table_methods;
pub mod json;

use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server, header::CONTENT_TYPE, service::{make_service_fn, service_fn}};
use tungstenite::http::{Method, StatusCode};
use std::convert::Infallible;
use serde::Serialize;

use crate::config;

#[derive(Serialize)]
struct Res {
    status: bool,
    message: String
}

pub fn build_response(data: &str) -> Response<Body> {
    Response::builder()
        .body(Body::from(data.to_string()))
        .unwrap()
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        // Handle "/row" path
        (&Method::GET, "/row") => {
            return row_methods::get(req).await;
        }

        (&Method::POST, "/row") => {
            row_methods::add(req).await
        }

        // (&Method::PUT, "/row") => {
        //     return methods::edit(req);
        // }

        (&Method::DELETE, "/row") => {
            return row_methods::delete(req).await;
        }

        (&Method::POST, "/bunch") => {
            return row_methods::bunch(req).await;
        }

        (&Method::GET, "/table") => {
            return table_methods::get(req).await;
        }

        (&Method::POST, "/table") => {
            return table_methods::create(req).await;
        }

        (&Method::DELETE, "/table") => {
            return table_methods::delete(req).await;
        }

        // (&Method::POST, "/db") => {
        //     return methods::delete(req);
        // }

        // (&Method::DELETE, "/db") => {
        //     return methods::delete(req);
        // }

        // Handle all other paths
        _ => {
            // Return a 404 Not Found response for unrecognized paths
            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap();
            Ok(response)
        }
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
pub async fn start() {
    // Create a new `Service` to handle incoming requests
    let make_svc = make_service_fn(|_conn| {
        async move {
            Ok::<_, Infallible>(service_fn(handle_request))
        }
    });

    // Create a new server and bind it to an address
    println!("HTTP server started at {}", config::CONFIG.port);
    let addr = SocketAddr::from(([0, 0, 0, 0], config::CONFIG.port));
    let server = Server::bind(&addr).serve(make_svc);

    // Start the server and await its completion
    match server.await {
        Ok(()) => (),
        Err(e) => eprintln!("Server error: {}", e),
    }
}

// pub fn start() {
//     run()
//     // thread::spawn(run);
// }
