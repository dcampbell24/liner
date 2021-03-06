extern crate liner;
extern crate termion;

use std::mem::replace;
use std::env::current_dir;

use liner::{Context, CursorPosition, Event, EventKind, FilenameCompleter};

fn main() {
    let mut con = Context::new();

    loop {
        let res = con.read_line("[prompt]$ ",
                       &mut |Event { editor, kind }| {
                if let EventKind::BeforeComplete = kind {
                    let (_, pos) = editor.get_words_and_cursor_position();

                    // Figure out of we are completing a command (the first word) or a filename.
                    let filename = match pos {
                        CursorPosition::InWord(i) => i > 0,
                        CursorPosition::InSpace(Some(_), _) => true,
                        CursorPosition::InSpace(None, _) => false,
                        CursorPosition::OnWordLeftEdge(i) => i >= 1,
                        CursorPosition::OnWordRightEdge(i) => i >= 1,
                    };

                    if filename {
                        let completer = FilenameCompleter::new(Some(current_dir().unwrap()));
                        replace(&mut editor.context().completer, Some(Box::new(completer)));
                    } else {
                        replace(&mut editor.context().completer, None);
                    }
                }
            })
            .unwrap();

        if res.is_empty() {
            break;
        }
        con.history.push(res.into());
    }
}
