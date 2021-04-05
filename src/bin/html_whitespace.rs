use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut src_file = PathBuf::new();
    src_file.push(args[0].to_owned());
    html_whitespace::minify_html(src_file);
}
