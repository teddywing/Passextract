extern crate clipboard;

use std::error::Error;

use clipboard::ClipboardContext;

pub struct ClipboardStore {
    context: ClipboardContext,
    last: String,
}

impl ClipboardStore {
    pub fn new() -> Result<ClipboardStore, Box<Error>> {
        let context = try!(ClipboardContext::new());

        return Ok(
            ClipboardStore {
                context: context,
                last: String::new(),
            }
        )
    }

    pub fn set_contents(&mut self, data: String) -> Result<(), Box<Error>> {
        // Save last value
        self.last = try!(self.context.get_contents());
        println!("last: {}", self.last);

        // Set new clipboard contents
        // self.context.set_contents(data)
        Ok(())
    }

    pub fn reset() {
    }
}
