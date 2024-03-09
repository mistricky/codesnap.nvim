const MIN_WIDTH: f32 = 100.;

fn min_width(width: f32) -> f32 {
    if width < MIN_WIDTH {
        MIN_WIDTH
    } else {
        width
    }
}

pub fn calc_wh(code: &str, char_wdith: f32, line_height: f32) -> (f32, f32) {
    let lines = code.lines();
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
