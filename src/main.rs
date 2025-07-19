use std::fs;
use std::env;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let mut args_iter = env::args()
        .collect::<Vec<String>>()
        .into_iter();
    args_iter.next();
 
    let mut essential_args = Vec::<String>::with_capacity(2);
    let mut title: Option<String> = None;

    loop {
        if let Some(arg) = args_iter.next() {
            if arg == "--title" { title = Some(args_iter.next().expect("No title specified.")); } 
            else { essential_args.push(arg) }
        } else { break; }
    }

    let input: String = fs::read_to_string(&essential_args[0])
        .expect("Error reading file to string");
    let mut output = String::new();
    for line in input.lines() {
        let converted = convert_line(&line, &mut title);
        output.push_str(&converted);
    }
    match title {
        Some(exp) => output = fill_html(&exp, &output),
        None => output = fill_html("Untitled", &output)
    }
    fs::write(&essential_args[1], output)
        .expect("Error writing to specified output file");
}

fn fill_html(page_title: &str, heading: &str) -> String {
    format!(
    "<!DOCTYPE html>
<html lang=\"en\">
    <head>
        <meta charset=\"UTF-8\">
        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
        <title>{}</title>
    </head>
    <body>{}</body>
</html>",
    page_title, heading)
}

fn convert_line(line: &str, title: &mut Option<String>) -> String {
    let line_graphemes = line.graphemes(true).collect::<Vec<&str>>();
    if (line_graphemes[0] == "#") && (line_graphemes[1] == " ") {
        match *title {
            None => { *title = Some(line_graphemes[1..].concat().trim().to_string()); },
            Some(_) => { }
        }
        return format!("<h1>{}</h1>", (&line_graphemes[1..]).concat().trim());

    } else {
        return format!("<p>{}</p>", line);
    }
}
