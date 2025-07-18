use std::fs;
use std::env;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Command requires an input file to process.");
        return;
    } else if args.len() < 3 {
        println!("Command requires an output file to process.");
        return;
    }

    if args.len() > 3 {
        println!("Received unecessary command line argument(s). Disregarding additional argument(s).");
    }

    let input: String = fs::read_to_string(&args[1])
        .expect("Error reading file to string");
    let mut output = String::new();
    input.lines()
        .map(convert_line)
        .for_each(|line: String| -> ()  { output.push_str(&line) });
    let output = fill_html("Test Title", &output);
    fs::write(&args[2], output)
        .expect("Error writing to specified output file");
}

fn fill_html(title: &str, heading: &str) -> String {
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
    title, heading)
}

fn convert_line(line: &str) -> String {
    let line_graphemes = line.graphemes(true).collect::<Vec<&str>>();
    if (line_graphemes[0] == "#") && (line_graphemes[1] == " ") {
        return format!("<h1>{}</h1>", (&line_graphemes[2..]).concat());
    } else {
        return format!("<p>{}</p>", line);
    }
}
