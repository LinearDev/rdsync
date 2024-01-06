use crate::{cache::cache_db, http::receiver::RequestHeaders};

/// Deletes a database based on the information provided in the request headers.
///
/// # Arguments
///
/// * `req` - RequestHeaders containing information about the database to be deleted.
///
/// # Returns
///
/// Returns a Result indicating the status of the database deletion operation.
/// - If the deletion is successful, it returns an empty string.
/// - If the database does not exist, it returns an error message indicating that the database doesn't exist.
/// - If an error occurs during the deletion, it returns an empty string.
pub fn delete(req: &RequestHeaders) -> Result<String, String> {
    return cache_db::delete_db(&req.db);
}