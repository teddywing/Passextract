extern crate rustty;

use rustty::{Terminal, Event, Cell, Color, Attr};
use rustty::ui::Painter;

use std::time::Duration;

struct Selection<'a> {
    x: usize,
    y: usize,
    terminal: &'a mut Terminal,
}

impl<'a> Selection<'a> {
    fn move_selection(&mut self, amount: usize) {
        // Clear old selection cursor
        self.terminal.printline(self.x, self.y, "  ");

        self.y = self.y + amount;

        let knockout_cell = Cell::with_style(Color::Byte(0x07), Color::Black, Attr::Default);
        self.terminal.printline_with_cell(0, 2, "->", knockout_cell);
    }
}

fn main() {
    let mut term = Terminal::new().unwrap();
    term.swap_buffers().unwrap();

    let knockout_cell = Cell::with_style(Color::Byte(0x07), Color::Black, Attr::Default);

    let selection = Selection { x: 2, y: 2, terminal: &mut term };

    loop {
        term.printline(0, 0, "Passextract (Press q or Ctrl-C to quit)");

        term.printline_with_cell(0, 2, "->", knockout_cell);
        term.printline(5, 2, "test");

        let options = vec![
            "e: booya@kacha.ch",
            "u: booyakacha",
            "p: secret"
        ];

        for (i, s) in options.iter().enumerate() {
            term.printline(5, i + 3, s)
        }

        term.set_cursor(2, 2).unwrap();

        let evt = term.get_event(Duration::from_millis(100)).unwrap();
        if let Some(Event::Key(ch)) = evt {
            match ch {
                'q' | '\x03' => {
                    break;
                }
                'j' => {
                    
                }
                'k' => {
                }
                c @ _ => {
                }
            }
        }

        term.swap_buffers().unwrap();
    }
}
