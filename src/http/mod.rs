pub mod row_methods;
pub mod table_methods;

use std::{net::SocketAddr, thread};
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

pub fn err_resp (message: &str) -> Response<Body> {
    let resp_id = Res {
        status: false,
        message: format!("ERROR: [http] {}", message)
    };
    let serialized = serde_json::to_string(&resp_id).unwrap();
    let response = Response::builder()
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(serialized)).unwrap();

    return response;
}

pub fn ok_resp (message: &str) -> Response<Body> {
    let resp_id = Res {
        status: true,
        message: format!("LOG: [http] {}", message)
    };
    let serialized = serde_json::to_string(&resp_id).unwrap();
    let response = Response::builder()
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(serialized)).unwrap();

    return response;
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        // Handle "/row" path
        (&Method::GET, "/row") => {
            return row_methods::get(req).await;
        }

        (&Method::POST, "/row") => {
            return row_methods::add(req).await;
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

#[tokio::main]
async fn run() {
    // Create a new `Service` to handle incoming requests
    let make_svc = make_service_fn(|_conn| {
        async {
            // Return the handler function for each request
            Ok::<_, Infallible>(service_fn(handle_request))
        }
    });

    // Create a new server and bind it to an address
    println!("HTTP server started at {}", config::CONFIG.port);
    let addr = SocketAddr::from(([0, 0, 0, 0], config::CONFIG.port));
    let server = Server::bind(&addr).serve(make_svc);

    // Start the server and await its completion
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}

pub fn start() {
    run()
    // thread::spawn(run);
}
