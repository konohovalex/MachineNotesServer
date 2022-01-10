use serde::Serialize;

#[derive(Serialize)]
pub struct Note {
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
        id: String,
        content: String,
    },
    
    #[serde(rename(serialize = "image"))]
    Image {
        id: String,
        #[serde(rename(serialize = "contentUrl"))]
        content_url: String,
    },

    #[serde(rename(serialize = "audio"))]
    Audio {
        id: String,
        #[serde(rename(serialize = "contentUrl"))]
        content_url: String,
    },
}
