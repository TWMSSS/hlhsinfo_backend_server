use std::io::Cursor;
use rocket::{response::Responder, Response, http::ContentType};

pub struct FileResponse {
    pub content_type: ContentType,
    pub file: Vec<u8>
}

impl<'r> Responder<'r, 'r> for FileResponse {
    fn respond_to(self, _request: &'r rocket::Request<'_>) -> rocket::response::Result<'r> {
        Response::build()
            .header(self.content_type)
            .sized_body(self.file.len(), Cursor::new(self.file))
            .ok()
    }
}