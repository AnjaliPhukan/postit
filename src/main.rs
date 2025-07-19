use std::fs;
use std::env;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let mut args_iter = env::args()
        .collect::<Vec<String>>()
        .into_iter();
    args_iter.next(); // consume cli tool name
 
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
    if line_graphemes[0..7].concat() == format!("{}{}", "#".repeat(6), " ") {
        return format!("<h6>{}</h6>", (&line_graphemes[6..]).concat().trim());
    }
    if line_graphemes[0..6].concat() == format!("{}{}", "#".repeat(5), " ") {
        return format!("<h5>{}</h5>", (&line_graphemes[5..]).concat().trim());
    }
    if line_graphemes[0..5].concat() == format!("{}{}", "#".repeat(4), " ") {
        return format!("<h4>{}</h4>", (&line_graphemes[4..]).concat().trim());
    }
    if line_graphemes[0..4].concat() == format!("{}{}", "#".repeat(3), " ") {
        return format!("<h3>{}</h3>", (&line_graphemes[3..]).concat().trim());
    }
    if line_graphemes[0..3].concat() == format!("{}{}", "#".repeat(2), " ") {
        return format!("<h2>{}</h2>", (&line_graphemes[2..]).concat().trim());
    }
    if (line_graphemes[0] == "#") && (line_graphemes[1] == " ") {
        match *title {
            None => { *title = Some(line_graphemes[1..].concat().trim().to_string()); },
            Some(_) => { }
        }
        return format!("<h1>{}</h1>", (&line_graphemes[2..]).concat().trim());
    }
    return format!("<p>{}</p>", line);
}
