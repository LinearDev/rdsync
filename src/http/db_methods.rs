use crate::{cache::cache_db, http::receiver::RequestHeaders};

pub fn delete(req: &RequestHeaders) -> Result<String, String> {
    return cache_db::delete_db(&req.db);
}