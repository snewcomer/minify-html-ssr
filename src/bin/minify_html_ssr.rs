use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut src_file = PathBuf::new();
    src_file.push(args[0].to_owned());
    let html = std::fs::read(src_file).unwrap();
    let res = minify_html_ssr::minify_html(html);
    let str_result = std::str::from_utf8(&res).unwrap();
    println!("count: {}", &str_result.chars().count());
    println!("{}", &str_result);
}
