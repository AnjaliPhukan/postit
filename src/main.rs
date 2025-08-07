use std::env;
use std::fs;
use std::io::ErrorKind;

fn main() {
    let mut args_iter = env::args().collect::<Vec<String>>().into_iter();
    args_iter.next(); // consume cli tool name

    let mut essential_args = Vec::<String>::with_capacity(2);
    let mut title: Option<String> = None;
    // loops over command line arguments to see if any of the optional
    // arguments have been passed
    loop {
        if let Some(arg) = args_iter.next() {
            if arg == "--title" || arg == "-t" {
                title = Some(args_iter.next().expect("No title specified."));
            } else if arg == "--help" {
                print_help();
                return;
            } else {
                match essential_args.len() < 2 {
                    true => { essential_args.push(arg); }
                    false => { println!("Additional non-optional argument \"{}\" supplied. Discarding argument. Disregarding argument.", &arg); }
                }
            }
        } else {
            break;
        }
    }

    match essential_args.len() {
        0 => {
            println!("Must specify both an input and output file.");
            return;
        },
        1 => {
            println!("Must specify an output file.");
            return;
        },
        _ => {}
    }


    let input = match fs::read_to_string(&essential_args[0]) {
        Ok(data) =>  data,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => { println!("No such input file \"{}\" found.", &essential_args[0]); return; },
            _ => { println!("Unrecoverable error when reading input file."); return; }
        }
    };
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

fn print_help() {
    println!("postit - used to parse a markdown page into an HTML page.
Usage: postit [OPTIONS] input output
Options:
    -t, --title         Set the title of the HTML page. Overrides auto h1 title(s).
");
}

/* Fills out a basic HTML template with the content generated during the 
 * */
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

fn resolve_special(line: &str) -> String {
    let mut converted = String::new();
    let mut special_stack = Vec::<char>::new();
    let symbols = line.chars().collect::<Vec<char>>();
    for (i, sym) in symbols.iter().enumerate() {
        if *sym == '*' {
            match special_stack.last() {
                Some(&top_sym) => {
                    if top_sym == '*' {
                        special_stack.pop();
                        converted.push_str("</em>");
                        continue;
                    } else {
                        special_stack.push('*');
                        converted.push_str("<em>");
                        continue;
                    }
                },
                None => {
                    special_stack.push('*');
                    converted.push_str("<em>");
                    continue;
                }
            }
        }
        converted.push(*sym);
    }
    return converted;
}

fn resolve_id(line: &str) -> Option<&str> {
    let mut found_curly_start = false;
    let mut start = 0;
    for (i, sym) in line.chars().enumerate() {
        if sym == '{' {
            found_curly_start = true;
            start = i;
        }
        if (sym == '}') && (found_curly_start == true) {
            return Some(&line[start..=i]);
        }
    }
    return None;
}

fn resolve_class(line: &str) -> Option<&str> {
    return None;
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

fn construct_tag_full(rest: &str, tag_type: String) -> String {
    return format!(
        "<{tag} class=\"{class}\" id=\"{id}\">{content}</{tag}>",
        tag = tag_type,
        id = "test",
        content = &resolve_special(rest),
        class = "s"
    );
}

fn convert_line(line: &str, title: &mut Option<String>) -> String {
    let (first_chunk, rest) = split_first_space(line);
    match first_chunk {
        "######" => { return format!("<h6>{}</h6>", &resolve_special(rest)); },
        "#####" => { return format!("<h5>{}</h5>", &resolve_special(rest)); },
        "####" =>  { return format!("<h4>{}</h4>", &resolve_special(rest)); },
        "###" =>  { return format!("<h3>{}</h3>", &resolve_special(rest)); },
        "##" =>  { return format!("<h2>{}</h2>", &resolve_special(rest)); },
        "#" => { 
            match *title {
                None => { *title = Some(String::from(rest)); },
                Some(_) => { }
            }
            return format!("<h1>{}</h1>", &resolve_special(rest));
        },
        _ => { return format!("<p>{}</p>", &resolve_special(rest)); }
    }
}
