use crate::files::FileMan;
use ivcnotes::asset::Terms;
use ivcnotes::Error;
use ivcnotes::{circuit::concrete::Concrete, note::NoteHistory};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct Notebook(Vec<NoteHistory<Concrete>>);

impl Notebook {
    pub(crate) fn create() -> Result<(), Error> {
        match FileMan::read_notebook() {
            Ok(_) => Ok(()),
            Err(_) => {
                let empty_notebook = Notebook::default();
                FileMan::write_notebook(&empty_notebook)
            }
        }
    }

    pub(crate) fn get_notes() -> Result<Vec<NoteHistory<Concrete>>, Error> {
        Ok(FileMan::read_notebook()?.0)
    }

    pub(crate) fn add_note(note: NoteHistory<Concrete>) -> Result<(), Error> {
        if !Self::note_exists(&note)? {
            let mut book = FileMan::read_notebook()?;
            book.0.push(note);
            FileMan::write_notebook(&book)?;
        }
        Ok(())
    }

    pub(crate) fn update_note(idx: usize, note: NoteHistory<Concrete>) -> Result<(), Error> {
        let mut book = FileMan::read_notebook()?;
        book.0[idx] = note;
        FileMan::write_notebook(&book)?;
        Ok(())
    }

    pub(crate) fn note_exists(note: &NoteHistory<Concrete>) -> Result<bool, Error> {
        let existing_notes = Self::get_notes()?;

        for existing_note in existing_notes {
            if existing_note.current_note == note.current_note
                && existing_note.sibling == note.sibling
            {
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub(crate) fn list_notes() -> Result<(), Error> {
        let notes = Self::get_notes()?;

        for (index, note) in notes.iter().enumerate() {
            let asset_type = match &note.asset.terms {
                Terms::IOU { unit, .. } => unit,
                // Add other asset types here if needed
            };

            let value = note.current_note.value;
            let step = note.current_note.step;
            let sender = &note.steps.last().unwrap().sender;

            println!("Note {}", index);
            println!("  Asset Type: {:#?}", asset_type);
            println!("  Value: {}", value);
            println!("  Step: {}", step);
            println!("  Sender: {:#?}", sender);
            println!();
        }

        Ok(())
    }
}
