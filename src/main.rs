use std::env;
use std::fs;

fn main() {
    let mut args_iter = env::args().collect::<Vec<String>>().into_iter();
    args_iter.next(); // consume cli tool name

    let mut essential_args = Vec::<String>::with_capacity(2);
    let mut title: Option<String> = None;

    loop {
        if let Some(arg) = args_iter.next() {
            if arg == "--title" || arg == "-t" {
                title = Some(args_iter.next().expect("No title specified."));
            } else {
                essential_args.push(arg);
            }
        } else {
            break;
        }
    }

    let input: String =
        fs::read_to_string(&essential_args[0]).expect("Error reading file to string");
    let mut output = String::new();
    for line in input.lines() {
        let converted = convert_line(line, &mut title);
        output.push_str(&converted);
    }
    match title {
        Some(exp) => output = fill_html(&exp, &output),
        None => output = fill_html("Untitled", &output),
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
        page_title, heading
    )
}

fn split_first_space(line: &str) -> (&str, &str) {
    let symbols = line.chars().collect::<Vec<char>>();
    for (i, sym) in symbols.iter().enumerate() {
        if *sym == ' ' {
            match symbols.get(i+1) {
                Some(_) => { return (&line[0..i], &line[i+1..]); },
                None => { return (&line[0..i], ""); }
            }
        }
    }
    return (&line[0..], "");
}

fn convert_line(line: &str, title: &mut Option<String>) -> String {
    let (first_chunk, rest) = split_first_space(line);
    match first_chunk {
        "######" => { return format!("<h6>{}</h6>", rest); },
        "#####" => { return format!("<h5>{}</h5>", rest); },
        "####" =>  { return format!("<h4>{}</h4>", rest); },
        "###" =>  { return format!("<h3>{}</h3>", rest); },
        "##" =>  { return format!("<h2>{}</h2>", rest); },
        "#" => { 
            match *title {
                None => { *title = Some(String::from(rest)); },
                Some(_) => { }
            }
            return format!("<h1>{}</h1>", rest);
        },
        _ => { return format!("<p>{}</p>", rest); }
    }
}
