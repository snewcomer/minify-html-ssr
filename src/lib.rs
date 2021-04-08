use wasm_bindgen::prelude::*;

// A-Za-Z
// exclamation mark !
// solidus /
// question mark ?
fn is_ascii_alpha(c: char) -> bool {
    match c {
        '\u{0041}'..='\u{005A}' => true,
        '\u{0061}'..='\u{007A}' => true,
        '\u{0021}' => true,
        '\u{002F}' => true,
        '\u{003F}' => true,
        _ => false,
    }
}

fn is_newline(c: char) -> bool {
    match c {
        '\u{000A}' => true,
        '\u{000D}' => true,
        _ => false,
    }
}

// TODO:
// Create Processor to encapsulate consuming characters and infinite number whitespaces
#[wasm_bindgen]
pub fn minify_html(in_html: Vec<u8>) -> Vec<u8> {
    // peekable allows us to get next item.  It does not allow us to peek at the previous nor by index
    let mut sliced_html = in_html.iter().peekable();
    let mut res = Vec::with_capacity(in_html.len());

    let mut inside_tag = false;
    let mut previous_tag = b' ';

    loop {
        match sliced_html.peek()  {
            Some(c) if inside_tag == false && is_newline(**c as char) => {
                previous_tag = **c;
            }, // this collapses whitespaces
            Some(c) if inside_tag == false && (previous_tag as char).is_whitespace() && (**c as char).is_whitespace() => {
                previous_tag = **c;
            }, // this collapses whitespaces
            Some(b'<') if inside_tag != true => {
                // consume '<' char
                sliced_html.next();

                // check following chars to see if valid html entity
                // e.g. < pre vs. < 2
                let mut next_char = sliced_html.peek();
                if next_char.is_none() {
                    // this is an error
                    break;
                } else {
                    // From the spec: https://html.spec.whatwg.org/multipage/parsing.html#tag-open-state
                    // After a `<`, a valid character is an ASCII alpha, `/`, `!`, or `?`. Anything
                    // else and the `<` is treated as content.
                    let mut tmp = vec![];
                    let mut ch = **next_char.unwrap();

                    while (ch as char).is_whitespace() {
                        tmp.push(b' ');

                        // consume
                        sliced_html.next();

                        next_char = sliced_html.peek();
                        ch = **next_char.unwrap();
                    }

                    // once we have consumed whitespaces, we can check spec
                    match is_ascii_alpha(ch as char) {
                        true => {
                            res.push(b'<');
                            inside_tag = true;
                        }
                        false => {
                            for l in "&lte".as_bytes() {
                                res.push(*l);
                            }
                        }
                    }

                    for c in tmp {
                        res.push(c);
                    }

                    previous_tag = ch;
                    res.push(ch);
                }
            }
            Some(b'>') if inside_tag == true => {
                res.push(b'>');
                inside_tag = false;
                previous_tag = b'>';
            }
            Some(c) => {
                res.push(**c);
                previous_tag = **c;
            }
            None => break
        };

        sliced_html.next();
    }

    res
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use super::*;

    #[test]
    fn it_works() {
        let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        dir.push("test/test-input.html");
        let html = fs::read(dir).unwrap();
        let result = minify_html(html);
        let str_result = std::str::from_utf8(&result).unwrap();
        assert_eq!(str_result, "<!DOCTYPE html><html dir=\"ltr\" lang=\"en-us\" xml:lang=\"en-us\"><body><!-- test --><h1 id=\"\">HI&nbsp;</h1><p>1 &lte 2</p>< p >2 &lte 4</p><p id=\"foo\" class=\"<\" aria-label=\"bar\">3 &lte 5</p><span /><div>hi <span class=\"bold\"> scott </span> a</div><!--%+b:8%-->0<!--%-b:8%--><music-video-player></music-video-player>style {height: 100;}</body></html>".to_owned());
    }
}
