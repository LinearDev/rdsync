use crate::http::{row_methods, receiver::RequestHeaders, table_methods, db_methods};

/// Handles incoming requests based on the provided path.
///
/// # Arguments
///
/// * `path` - The type of the request.
/// * `head` - The request headers.
/// * `body` - The request body.
///
/// # Returns
///
/// * `Result<String, String>` - A `Result` containing the response or an error message.
///
/// # Examples
///
/// ```rust
/// let result = handle_request("get_row", &request_headers, "request_body");
/// match result {
///     Ok(response) => println!("Response: {}", response),
///     Err(error) => println!("Error: {}", error),
/// }
/// ```
pub fn handle_request(path: &str, head: &RequestHeaders, body: &str) -> Result<String, String> {
    println!("[ INFO ]: get new request - `{}`", path);

    match path {
        // Handle "/row" path
        "get_row" => {
            return row_methods::get(head);
        }

        "add_row" => {
            return row_methods::add(head, &body);
        }

        // "edit_row" => {
        //     return row_methods::edit(req);
        // }

        "delete_row" => {
            return row_methods::delete(head);
        }

        "add_bunch" => {
            return row_methods::bunch(head, &body);
        }

        "get_table" => {
            return table_methods::get(head);
        }

        "get_table_data" => {
            return table_methods::get_with_keys(head);
        }

        "add_table" => {
            return table_methods::create(head);
        }

        "delete_table" => {
            return table_methods::delete(head);
        }

        // (&Method::POST, "/db") => {
        //     return methods::delete(req);
        // }

        "delete_db" => {
            return db_methods::delete(head);
        }

        /// Handle all other paths
        _ => {
            /// Return a `no action` Not Found response for unrecognized paths
            Ok("no action".to_string())
        }
    }
}
