use super::notes_data::{DateTime, Note, NoteContent};
use rand::Rng;

static DUMMY_NOTE_TEXT: &str =
    "Заметка с очень длинным названием, которое не поместилось вот чуть-чуть\n
И был этот текст крайне странным, и длина его превышала все границы адекватности\n
В общем:\n
Побывал я тут в Европе и повидал множество дивных вещей: архитектуру, людей";

pub fn create_dummy_notes(items_amount: i32) -> Vec<Note> {
    let mut notes_vec = Vec::new();

    let mut random_generator = rand::thread_rng();

    for id in 0..items_amount {
        let mode = random_generator.gen_range(0..=2);

        notes_vec.push(create_dummy_note(id.to_string(), mode))
    }

    return notes_vec;
}

pub fn create_dummy_note(id: String, mode: i32) -> Note {
    let note_content_order_number = 1;
    let note_content_id = format!("{}_{}", id, note_content_order_number);
    let date_time_created = DateTime {};
    let date_time_last_edited = DateTime {};

    return Note {
        id,
        date_time_created,
        date_time_last_edited,
        note_content: {
            let mut note_content_vec: Vec<NoteContent> = Vec::new();
            note_content_vec.push(match mode {
                0 => NoteContent::Text {
                    id: note_content_id,
                    content: String::from(DUMMY_NOTE_TEXT),
                },
                1 => NoteContent::Image {
                    id: note_content_id,
                    content_url: String::from(""),
                },

                _ => NoteContent::Audio {
                    id: note_content_id,
                    content_url: String::from(""),
                },
            });
            note_content_vec
        },
    };
}
