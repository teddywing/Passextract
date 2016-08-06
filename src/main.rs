extern crate clipboard;
extern crate rustty;

use clipboard::ClipboardContext;

use rustty::{Terminal, Event, Cell, Color, Attr};
use rustty::ui::Painter;

use std::env;
use std::io::{self, BufRead};
use std::process::{self, Command};
use std::time::Duration;

struct Point {
    x: usize,
    y: usize,
}

/// Strips the part of a line before and including the first colon.
///
/// # Examples
///
/// ```
/// let value = strip_key("e: email@example.com");
/// assert_eq!(value, "email@example.com");
/// ```
fn strip_key(line: &str) -> &str {
    let strings: Vec<&str> = line.split(": ").collect();
    strings[1]
}

fn move_selection(term: &mut Terminal, selection: &mut Point, style: Cell, amount: isize) {
    term.printline(selection.x, selection.y, "  ");

    let y = selection.y as isize;
    selection.y = (y + amount) as usize;

    term.printline_with_cell(selection.x, selection.y, "->", style);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = if args.len() > 1 {
        &args[1]
    } else {
        "-"
    };

    let file = Command::new("pass")
        .arg("show")
        .arg(input)
        .output()
        .expect("Error executing `pass`")
        .stdout;

    let output = String::from_utf8_lossy(&file);
    let mut options = Vec::new();
    for line in output.lines() {
        if line.starts_with("e: ") ||
            line.starts_with("u: ") ||
            line.starts_with("p: ") {
            options.push(line);
        }
    }

    // let mut options = Vec::new();
    // let stdin = io::stdin();
    // for line in stdin.lock().lines() {
    //     let line = line.expect("Error reading from STDIN");
    //
    //     if line.starts_with("e: ") ||
    //         line.starts_with("u: ") ||
    //         line.starts_with("p: ") {
    //         options.push(line);
    //     }
    // }

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
        term.printline_with_cell(0, 0, "Passextract (Press q or Ctrl-C to quit)", knockout_cell);

        term.printline_with_cell(selection.x, selection.y, "->", knockout_cell);

        for (i, s) in options.iter().enumerate() {
            term.printline(5, i + 2, s)
        }

        let evt = term.get_event(Duration::from_millis(100)).unwrap();
        if let Some(Event::Key(ch)) = evt {
            match ch {
                'q' | '\x03' => {
                    break;
                }
                'j' => {
                    if selection.y < options.len() + 1 {
                        move_selection(&mut term, &mut selection, knockout_cell, 1)
                    }
                }
                'k' => {
                    if selection.y > 2 {
                        move_selection(&mut term, &mut selection, knockout_cell, -1)
                    }
                }
                '\x0D' => {
                    match clipboard_ctx.set_contents(strip_key(&options[selection.y - 2]).to_owned()) {
                        Ok(_) => {
                            term.printline_with_cell(selection.x, selection.y, "->", green_cell);
                        },
                        Err(_) => {
                            term.printline_with_cell(selection.x, selection.y, "->", red_cell);
                        }
                    }
                }
                _ => { continue }
            }
        }

        term.swap_buffers().unwrap();
    }
}
