use regex::Regex;

const MIN_WIDTH: f32 = 100.;

fn min_width(width: f32) -> f32 {
    if width < MIN_WIDTH {
        MIN_WIDTH
    } else {
        width
    }
}

pub fn calc_wh(text: &str, char_wdith: f32, line_height: f32) -> (f32, f32) {
    let trimmed_text = trim_space(text);
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

pub fn trim_space(text: &str) -> String {
    let lines = text.split("\n").collect::<Vec<&str>>();
    let first_line = lines.first().unwrap();
    let head_spaces = Regex::new(r"^(\s*)").unwrap().find(first_line);

    match head_spaces {
        Some(head_spaces) => {
            return lines
                .into_iter()
                .map(|line| {
                    Regex::new(format!("^{}", head_spaces.as_str()).as_ref())
                        .unwrap()
                        .replace(line, "")
                        .to_string()
                })
                .collect::<Vec<String>>()
                .join("\n");
        }
        None => text.to_string(),
    }
}
