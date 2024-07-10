use crate::{
    code::{calc_max_line_number_length, calc_wh, prepare_code},
    config::TakeSnapshotParams,
};
use arboard::Clipboard;
#[cfg(target_os = "linux")]
use arboard::SetExtLinux;
use nvim_oxi::Result;
use std::cmp::max;

const SPACE_BOTH_SIDE: usize = 2;

fn optional(component: String, is_view: bool) -> String {
    if is_view {
        component
    } else {
        "".to_string()
    }
}

#[allow(dead_code)]
pub fn copy_ascii(params: TakeSnapshotParams) -> Result<()> {
    let code = prepare_code(&params.code);
    let (width, height) = calc_wh(&code, 1., 1.);
    let calc_line_number_width =
        |start_line_number: usize| calc_max_line_number_length(height as usize, start_line_number);
    let frame_width = max(width as usize, params.file_path.len()) + SPACE_BOTH_SIDE;
    let frame_width = match params.start_line_number {
        Some(start_line_number) => {
            frame_width + SPACE_BOTH_SIDE + calc_line_number_width(start_line_number)
        }
        None => frame_width,
    };
    let line = format!("│{}│\n", "─".repeat(frame_width));
    let frame_width_with_content = frame_width - 1;
    let top_frame = format!("╭{}╮\n", "─".repeat(frame_width));
    let bottom_frame = format!("╰{}╯", "─".repeat(frame_width));
    let code = code
        .lines()
        .enumerate()
        .map(|(i, line)| {
            format!(
                "│ {:1$} │\n",
                match params.start_line_number {
                    Some(start_line_number) => format!(
                        "{:1$} {line}",
                        start_line_number + i,
                        calc_line_number_width(start_line_number),
                    ),
                    None => line.to_string(),
                },
                frame_width_with_content - 1
            )
        })
        .collect::<String>();
    let text_line = |text: &str| format!("│ {:1$}│\n", text, frame_width_with_content);
    let breadcrumbs = optional(
        format!("{}{line}", text_line(&params.file_path)),
        params.has_breadcrumbs,
    );
    let ascii_snapshot = format!("{top_frame}{breadcrumbs}{code}{bottom_frame}");

    #[cfg(target_os = "linux")]
    std::thread::spawn(move || {
        Clipboard::new()
            .unwrap()
            .set()
            .wait()
            .text(ascii_snapshot)
            .unwrap();
    });

    #[cfg(not(target_os = "linux"))]
    Clipboard::new().unwrap().set_text(ascii_snapshot).unwrap();

    Ok(())
}
