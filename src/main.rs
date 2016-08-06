extern crate rustty;

use rustty::{Terminal, Event, Cell, Color, Attr};
use rustty::ui::Painter;

use std::time::Duration;

fn main() {
    let mut term = Terminal::new().unwrap();
    term.swap_buffers().unwrap();

    let knockout_cell = Cell::with_style(Color::Byte(0x07), Color::Black, Attr::Default);

    loop {
        term.printline(0, 0, "Passextract (Press q or Ctrl-C to quit)");

        term.printline_with_cell(0, 2, "->", knockout_cell);
        term.printline(5, 2, "test");

        term.set_cursor(2, 2).unwrap();

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
