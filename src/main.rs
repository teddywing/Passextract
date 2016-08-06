extern crate rustty;

use rustty::{Terminal, Event};
use rustty::ui::Painter;

use std::time::Duration;

fn main() {
    let mut term = Terminal::new().unwrap();
    term.swap_buffers().unwrap();

    loop {
        term.printline(0, 0, "Passextract (Press q or Ctrl-C to quit)");

        let evt = term.get_event(Duration::from_millis(100)).unwrap();
        if let Some(Event::Key(ch)) = evt {
            match ch {
                'q' | '\x03' => {
                    break;
                }
                c @ _ => {
                }
            }
        }

        term.swap_buffers().unwrap();
    }
}
