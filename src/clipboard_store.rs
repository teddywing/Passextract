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

    pub fn set_contents(&mut self, data: String) -> Result<(), Box<Error>> {
        if self.original.is_empty() {
            self.original = try!(self.context.get_contents())
        }

        // Set new clipboard contents
        // self.context.set_contents(data)
        Ok(())
    }

    pub fn reset() {
    }
}
