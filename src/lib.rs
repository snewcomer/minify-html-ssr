use wasm_bindgen::prelude::*;

// A-Za-Z
// exclamation mark !
// solidus /
// question mark ?
// codepoints
fn is_ascii_alpha(c: &char) -> bool {
    match c {
        '\u{0041}'..='\u{005A}' => true,
        '\u{0061}'..='\u{007A}' => true,
        '\u{0021}' => true,
        '\u{002F}' => true,
        '\u{003F}' => true,
        _ => false,
    }
}

fn is_newline(c: &char) -> bool {
    match c {
        '\u{000A}' => true,
        '\u{000D}' => true,
        _ => false,
    }
}

const LTE: &str = "&lte";
const OPEN_TAG: char = '<';
const CLOSE_TAG: char = '>';

// TODO:
// Create Processor to encapsulate consuming characters and infinite number whitespaces
#[wasm_bindgen]
pub fn minify_html(in_html: String) -> String {
    // iterate over chars, not bytes
    // we might be in the middle of a multi-byte
    let html = in_html.chars();

    // peekable allows us to get next item.  It does not allow us to peek at the previous nor by index
    let mut sliced_html = html.peekable();
    let mut res = String::with_capacity(in_html.len());

    let mut inside_tag = false;
    let mut previous_tag = ' ';

    loop {
        // peek gives us &&u8
        match sliced_html.peek() {
            Some(ch) if inside_tag == false && is_newline(ch) => {
                previous_tag = *ch;
            }, // this collapses whitespaces
            Some(ch) if inside_tag == false && previous_tag.is_whitespace() && ch.is_whitespace() => {
                previous_tag = *ch;
            }, // this collapses whitespaces
            Some('<') if inside_tag != true => {
                // consume '<' char
                sliced_html.next();

                // check following chars to see if valid html entity
                // e.g. < pre vs. < 2
                let mut next_ch = sliced_html.peek();
                if next_ch.is_none() {
                    // this is an error
                    break;
                } else {
                    // From the spec: https://html.spec.whatwg.org/multipage/parsing.html#tag-open-state
                    // After a `<`, a valid character is an ASCII alpha, `/`, `!`, or `?`. Anything
                    // else and the `<` is treated as content.
                    let mut tmp = vec![];
                    let mut ch = next_ch.unwrap();

                    while ch.is_ascii_whitespace() {
                        tmp.push(*ch);

                        // consume
                        sliced_html.next();

                        next_ch = sliced_html.peek();
                        ch = next_ch.unwrap();
                    }

                    // once we have consumed whitespaces, we can check spec
                    match is_ascii_alpha(ch) {
                        true => {
                            res.push(OPEN_TAG);
                            inside_tag = true;
                        }
                        false => {
                            res.push_str(LTE);
                        }
                    }

                    for ch in tmp {
                        res.push(ch);
                    }

                    previous_tag = *ch;
                    res.push(*ch);
                }
            }
            Some('>') if inside_tag == true => {
                res.push(CLOSE_TAG);
                inside_tag = false;
                previous_tag = '>';
            }
            Some(ch) => {
                res.push(*ch);
                previous_tag = *ch;
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
        let html = fs::read_to_string(dir).unwrap();
        let result = minify_html(html);
        assert_eq!(result, "<!DOCTYPE html><html dir=\"ltr\" lang=\"en-us\" xml:lang=\"en-us\"><body><!-- test --><h1 id=\"\">HI&nbsp;</h1><p>1 &lte 2</p>< p >2 &lte 4</p><p id=\"foo\" class=\"<\" aria-label=\"bar\">3 &lte 5</p><span /><div>hi <span class=\"bold\"> scott </span> a</div><!--%+b:8%-->0<!--%-b:8%--><music-video-player></music-video-player>style {height: 100;}</body></html>".to_owned());
    }

    #[test]
    fn bad_input() {
        let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        dir.push("test/test-input-bad.html");
        let html = fs::read_to_string(dir).unwrap();
        let result = minify_html(html);
        assert_eq!(result, "<!DOCTYPE html><html prefix=\"og: http://ogp.me/ns#\"  dir=\"ltr\" lang=\"en-us\" xml:lang=\"en-us\"><body\n\n  </body></html>".to_owned());
    }

    #[test]
    fn asian_chars() {
        let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        dir.push("test/test-asian-chars.html");
        let html = fs::read_to_string(dir).unwrap();
        let result = minify_html(html);
        assert_eq!(result, "<h1>異體字字典</h1>".to_owned());
    }
}
