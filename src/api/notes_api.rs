use super::data::paging_data::PaginationInfo;
use actix_web::{get, web::Json, web::Query, Responder};

use super::data::notes_data_dummy;

#[get("v1/notes")]
pub async fn get_notes(pagination_info: Query<PaginationInfo>) -> impl Responder {
    let notes = notes_data_dummy::create_dummy_notes(pagination_info.page_size);
    return Json(notes);
}
