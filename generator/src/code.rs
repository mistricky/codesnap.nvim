use cached::proc_macro::cached;
use regex::Regex;

const MIN_WIDTH: f32 = 100.;

fn min_width(width: f32) -> f32 {
    if width < MIN_WIDTH {
        MIN_WIDTH
    } else {
        width
    }
}

// Because the code block is input by users, we need to calculate the width and height
// to make sure render the width and height of the "editor" shape correctly
#[cached(key = "String", convert = r#"{ format!("{}", text) }"#)]
pub fn calc_wh(text: &str, char_wdith: f32, line_height: f32) -> (f32, f32) {
    let trimmed_text = prepare_code(text);
    let lines = trimmed_text.lines();
    let max_length_line = lines.clone().into_iter().fold("", |max_length_line, cur| {
        if cur.len() > max_length_line.len() {
            cur
        } else {
            max_length_line
        }
    });
    let width = max_length_line.len() as f32 * char_wdith;
    let height = lines.collect::<Vec<&str>>().len() as f32 * line_height;

    (min_width(width), height)
}

// The tab character is incorrectly render using cosmic, need to replace all tab with space
// before render the code
fn replace_tab_to_space(text: &str) -> String {
    let spaces = " ".repeat(2);

    str::replace(text, "\t", &spaces)
}

// Find min indention of code lines, and remove the same indention from subsequent lines
fn trim_space(text: &str) -> String {
    let lines = text.split("\n").collect::<Vec<&str>>();
    let regex = Regex::new(r"(?:^|\n)(\s*)").unwrap();
    let captures_iter = regex.captures_iter(text);
    let space_lengths = captures_iter
        .map(|capture| capture.get(1).unwrap().as_str().len())
        .collect::<Vec<usize>>();

    if space_lengths.len() < lines.len() {
        return text.to_string();
    }

    let need_to_remove_spaces = " ".repeat(space_lengths.into_iter().min().unwrap());

    lines
        .into_iter()
        .map(|line| {
            Regex::new(format!("^{}", need_to_remove_spaces).as_ref())
                .unwrap()
                .replace(line, "")
                .to_string()
        })
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn prepare_code(code: &str) -> String {
    trim_space(&replace_tab_to_space(&code))
}
