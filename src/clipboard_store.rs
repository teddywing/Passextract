extern crate clipboard;

use std::error::Error;

use clipboard::ClipboardContext;

pub struct ClipboardStore {
    context: ClipboardContext,
    original: String,
}

impl ClipboardStore {
    pub fn new() -> Result<ClipboardStore, Box<Error>> {
        let context = try!(ClipboardContext::new());

        return Ok(
            ClipboardStore {
                context: context,
                original: String::new(),
            }
        )
    }

    /// Set the contents of the system clipboard. Stores the original contents
    /// of the clipboard the first time the function is run.
    pub fn set_contents(&mut self, data: String) -> Result<(), Box<Error>> {
        if self.original.is_empty() {
            self.original = try!(self.context.get_contents())
        }

        self.context.set_contents(data)
    }

    pub fn reset() {
    }
}
