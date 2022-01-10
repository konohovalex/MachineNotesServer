use serde::Deserialize;

#[derive(Deserialize)]
pub struct PaginationInfo {
    #[serde(rename(deserialize = "pageSize"))]
    pub page_size: i32,
    pub page: i32
}
