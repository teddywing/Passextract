// Copyright (c) 2016, 2018  Teddy Wing
//
// This file is licensed under the GNU GPLv3+. Please see the COPYING
// file for more information.

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
fn strip_key(line: &str) -> String {
    let strings: Vec<&str> = line.split(": ").collect();
    strings[1..].join(": ")
}

fn move_selection(term: &mut Terminal, selection: &mut Point, style: Cell, y: usize) {
    term.printline(selection.x, selection.y, "  ");

    selection.y = y;

    term.printline_with_cell(selection.x, selection.y, "->", style);
}

/// Given a filename, either parse options from STDIN or send the file to
/// `pass show` and parse the result as options.
fn parse_options(filename: &str) -> Vec<String> {
    fn push_option(options: &mut Vec<String>, line: String) {
        if line.starts_with("e: ") ||
            line.starts_with("u: ") ||
            line.starts_with("p: ") {
            options.push(line);
        }
    }

    let mut options = Vec::new();

    if filename == "-" {
        let stdin = io::stdin();

        for line in stdin.lock().lines() {
            let line = line.expect("Error reading from STDIN");
            push_option(&mut options, line.to_owned());
        }
    } else {
        let mut child = Command::new("pass")
            .arg("show")
            .arg(filename)
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect("Error executing `pass`");

        child.wait().expect("Error waiting for `pass`");

        let stdout = child.stdout.take().unwrap();

        // let file = stdout;
        // let stdout = child.stdout.unwrap();
        let file = io::BufReader::new(stdout);

        // let mut line = String::new();

        // loop {
        //     let n_bytes = file.read_line(&mut line).unwrap();
        //     if n_bytes == 0 {
        //         break;
        //     }
        //
        //     push_option(&mut options, line.to_owned());
        // }

        for line in file.lines() {
            push_option(&mut options, line.unwrap());
        }
    }

    options
}

/// Checks whether the given line starts with "p: "
///
/// # Examples
///
/// ```
/// assert_eq!(is_password_line("p: secret"), true);
/// ```
fn is_password_line(line: &str) -> bool {
    line.starts_with("p: ")
}

/// Replaces the password on a password line with "*"s.
///
/// # Examples
///
/// ```
/// assert_eq!(hide_password_line("p: secret"), "p: ******);
/// ```
fn hide_password_line(line: &str) -> String {
    const KEY_LENGTH: usize = 3;
    let password_length = line.len() - KEY_LENGTH;

    format!("p: {}", "*".repeat(password_length))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let options = ["-i"];

    let hide_password = match args.get(1) {
        Some(arg) if arg == "-i" => true,
        Some(_) => false,
        None => false,
    };

    let last_arg = &args[args.len() - 1];
    let input = if args.len() > 1 &&
            !options.contains(&last_arg.as_ref()) {
        last_arg
    } else {
        "-"
    };

    let options = parse_options(input);

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
            if hide_password && is_password_line(s) {
                term.printline(5, i + 2, &hide_password_line(s))
            } else {
                term.printline(5, i + 2, s)
            }
        }

        let evt = term.get_event(Duration::from_millis(100)).unwrap();
        if let Some(Event::Key(ch)) = evt {
            match ch {
                'q' | '\x03' => {
                    // Clear the clipboard on quit
                    match clipboard_ctx.set_contents("".to_owned()) {
                        Ok(_) => break,
                        Err(_) => continue,
                    }
                }
                'j' => {
                    if selection.y < options.len() + 1 {
                        let y = selection.y;
                        move_selection(&mut term, &mut selection, knockout_cell, y + 1)
                    }
                }
                'k' => {
                    if selection.y > 2 {
                        let y = selection.y;
                        move_selection(&mut term, &mut selection, knockout_cell, y - 1)
                    }
                }
                'g' => {
                    move_selection(&mut term, &mut selection, knockout_cell, 2)
                }
                'G' => {
                    move_selection(&mut term, &mut selection, knockout_cell, options.len() + 1)
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
