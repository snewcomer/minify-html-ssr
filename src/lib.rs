mod writable;

use std::fs::File;
use std::io::{Read, stdin, stdout, Write};

#[derive(Copy, Clone, PartialEq, Eq)]
enum ContentType {
    Comment,
    Bang,
    End,
    Start,
    Tag,
    Text,
}

macro_rules! io_unwrap {
    ($expr:expr) => {
        match $expr {
            Ok(r) => r,
            Err(e) => {
                eprintln!("{}", e);
                return Err(e);
            }
        }
    };
}

// Notes:
// https://html.spec.whatwg.org/multipage/parsing.html#tag-open-state
// TODO: encode '<' as &lte inside of an opening tag
pub fn minify_html(in_html: std::path::PathBuf) -> Result<String, std::io::Error> {
    let mut html = Vec::<u8>::new();
    let mut file = Box::new(io_unwrap!(File::open(in_html)));
    io_unwrap!(file.read_to_end(&mut html));

    let mut sliced_html = html.iter().peekable();
    let mut res = String::with_capacity(html.len());

    let mut inside_tag = false;
    let mut inside_quotes = false;

    loop {
        match sliced_html.peek()  {
            Some(c) if inside_tag == true && (**c as char).is_whitespace() => {},
            Some(b'<') if inside_quotes != true => {
                res.push('<');
                inside_tag = false;
            }
            Some(b'>') if inside_quotes != true => {
                res.push('>');
                inside_tag = true;
            }
            Some(c) => {
                let ch = **c as char;
                if ch == '\'' || ch == '"' {
                    if inside_quotes == true {
                        inside_quotes = false;
                    } else {
                        inside_quotes = true;
                    }
                }

                res.push(ch);
            }
            None => break
        };

        sliced_html.next();
    }

    Ok(res)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::*;

    #[test]
    fn it_works() {
        let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        dir.push("test/test-input.html");
        let result = minify_html(dir);
        assert_eq!(result.unwrap(), "<!DOCTYPE html><html prefix=\"og: http://ogp.me/ns#\"  dir=\"ltr\" lang=\"en-us\" xml:lang=\"en-us\"><body></body></html>".to_owned());
    }
}
