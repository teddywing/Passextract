extern crate clipboard;
extern crate rustty;

use clipboard::ClipboardContext;

use rustty::{Terminal, Event, Cell, Color, Attr};
use rustty::ui::Painter;

use std::io::{self, BufRead};
use std::process;
use std::time::Duration;

struct Point {
    x: usize,
    y: usize,
}

fn strip_key(line: &str) -> &str {
    let strings: Vec<&str> = line.split(": ").collect();
    strings[1]
}

fn main() {
    let mut options = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Error reading from STDIN");

        if line.starts_with("e: ") ||
            line.starts_with("u: ") ||
            line.starts_with("p: ") {
            options.push(line);
        }
    }

    if options.is_empty() {
        process::exit(1);
    }

    let mut term = Terminal::new().unwrap();
    term.swap_buffers().unwrap();

    let knockout_cell = Cell::with_style(Color::White, Color::Black, Attr::Default);
    let red_cell = Cell::with_style(Color::White, Color::Red, Attr::Default);
    let green_cell = Cell::with_style(Color::White, Color::Green, Attr::Default);

    let mut selection = Point { x: 0, y: 2 };

    let mut clipboard_ctx = ClipboardContext::new().unwrap();

    loop {
        term.printline(0, 0, "Passextract (Press q or Ctrl-C to quit)");

        term.printline_with_cell(selection.x, selection.y, "->", knockout_cell);
        term.printline(5, 2, "test");

        for (i, s) in options.iter().enumerate() {
            term.printline(5, i + 3, s)
        }

        term.set_cursor(selection.x + 2, selection.y).unwrap();

        let evt = term.get_event(Duration::from_millis(100)).unwrap();
        if let Some(Event::Key(ch)) = evt {
            match ch {
                'q' | '\x03' => {
                    break;
                }
                'j' => {
                    if selection.y < options.len() + 2 {
                        term.printline(selection.x, selection.y, "  ");

                        selection.y = selection.y + 1;

                        term.printline_with_cell(selection.x, selection.y, "->", knockout_cell);
                        term.set_cursor(selection.x + 2, selection.y).unwrap();
                    }
                }
                'k' => {
                    if selection.y > 2 {
                        term.printline(selection.x, selection.y, "  ");

                        selection.y = selection.y - 1;

                        term.printline_with_cell(selection.x, selection.y, "->", knockout_cell);
                        term.set_cursor(selection.x + 2, selection.y).unwrap();
                    }
                }
                '\x0D' => {
                    match clipboard_ctx.set_contents(strip_key(&options[selection.y - 3]).to_owned()) {
                        Ok(_) => {
                            term.printline_with_cell(selection.x, selection.y, "->", green_cell);
                        },
                        Err(_) => {
                            term.printline_with_cell(selection.x, selection.y, "->", red_cell);
                        }
                    }
                }
                c @ _ => {
                }
            }
        }

        term.swap_buffers().unwrap();
    }
}
