use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct PaginationInfo {
    #[serde(rename(deserialize = "pageSize"))]
    pub page_size: i32,
    pub page: i32,
}

#[derive(Serialize)]
pub struct Note {
    #[serde(rename(serialize = "id"))]
    pub id: String,
    #[serde(rename(serialize = "dateTimeCreated"))]
    pub date_time_created: DateTime,
    #[serde(rename(serialize = "dateTimeLastEdited"))]
    pub date_time_last_edited: DateTime,
    #[serde(rename(serialize = "noteContent"))]
    pub note_content: Vec<NoteContent>,
}

#[derive(Serialize)]
pub struct DateTime {}

#[derive(Serialize)]
pub enum NoteContent {
    #[serde(rename(serialize = "text"))]
    Text {
        #[serde(rename(serialize = "id"))]
        id: String,
        #[serde(rename(serialize = "content"))]
        content: String,
    },
    #[serde(rename(serialize = "image"))]
    Image {
        #[serde(rename(serialize = "id"))]
        id: String,
        #[serde(rename(serialize = "contentUrl"))]
        content_url: String,
    },

    #[serde(rename(serialize = "audio"))]
    Audio {
        #[serde(rename(serialize = "id"))]
        id: String,
        #[serde(rename(serialize = "contentUrl"))]
        content_url: String,
    },
}
