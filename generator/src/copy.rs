use crate::{config::TakeSnapshotParams, snapshot::take_snapshot};
#[cfg(target_os = "linux")]
use arboard::SetExtLinux;
use arboard::{Clipboard, ImageData};
use nvim_oxi::{lua::Error::RuntimeError, Error, Result};

pub fn copy_into_clipboard(config: TakeSnapshotParams) -> Result<()> {
    let pixmap = take_snapshot(config.clone())?;
    let premultplied_colors = pixmap.pixels();
    let colors = premultplied_colors
        .iter()
        .map(|premultplied_color| {
            vec![
                premultplied_color.red(),
                premultplied_color.green(),
                premultplied_color.blue(),
                premultplied_color.alpha(),
            ]
        })
        .flatten()
        .collect::<Vec<u8>>();

    let img_data = ImageData {
        width: pixmap.width() as usize,
        height: pixmap.height() as usize,
        bytes: colors.into(),
    };

    #[cfg(target_os = "linux")]
    if wsl::is_wsl() {
        let temp_dir = std::env::temp_dir();
        let filename = generate_random_filename();

        let path = format!("{}/{}", String::from(temp_dir.to_str().unwrap()), filename);
        let _ = pixmap
            .save_png(path.clone())
            .map_err(|err| Error::Lua(RuntimeError(err.to_string())));

        //getting mounted vdisk location of linux install
        let os_linux_release = sys_info::linux_os_release().unwrap();
        let mut wsl_path = format!(
            "\\\\wsl$\\{}",
            os_linux_release.pretty_name()
        );
        if !powershell_folder_exist(wsl_path.clone()) {
            wsl_path = format!(
                "\\\\wsl$\\{}",
                os_linux_release.name()
            );
        }
        let src_path = format!(
            "{}\\tmp\\{}",
            wsl_path,
            filename
        );

        let _ = copy_to_wsl_clipboard(&src_path);
        //delete the file when done?
    } else {
        std::thread::spawn(move || {
            Clipboard::new()
                .unwrap()
                .set()
                .wait()
                .image(img_data)
                .unwrap();
        });
    }
    #[cfg(not(target_os = "linux"))]
    Clipboard::new().unwrap().set_image(img_data).unwrap();

    Ok(())
}

fn copy_to_wsl_clipboard(src_path: &str) -> Result<()> {
    println!("{}", src_path);
    let powershell = Command::new("/mnt/c/Windows//System32/WindowsPowerShell/v1.0/powershell.exe")
        .arg("-NoProfile")
        .arg("-Command")
        .arg(&format!("Get-ChildItem \"{}\" | Set-Clipboard", src_path))
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();

    // Wait until the powershell process is finished before returning
    let _ = powershell.unwrap().wait().unwrap();

    Ok(())
}

use std::{
    process::{Command, Stdio}, time::Instant
};

fn powershell_folder_exist(src_path: String) -> bool {
    let powershell = Command::new("/mnt/c/Windows//System32/WindowsPowerShell/v1.0/powershell.exe")
        .arg("-NoProfile")
        .arg("-Command")
        .arg(&format!("Test-Path -path \"{}\"", src_path))
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output();

    let stdout = String::from_utf8(powershell.unwrap().stdout).unwrap();

    let result = stdout == "True\r\n";

    return result;
}

fn generate_random_filename() -> String {
    // Get nanoseconds since epoch for randomness
    let now = Instant::now();
    let random_part = format!("{:016x}", now.elapsed().as_nanos() % u128::MAX);

    // Combine prefix, random part, and extension
    format!("codesnap_{}.png", random_part)
}

#[cfg(test)]
mod test {
    use std::process::{Command, Stdio};

    use super::copy_to_wsl_clipboard;

    #[test]
    #[cfg(target_os = "linux")]
    fn copy_to_wsl_test() -> std::result::Result<(), nvim_oxi::Error> {
        
        if !wsl::is_wsl() {
            return Ok(());
        }

        //not sure about the best way to test this in a dynamic way
        //dir example
        //"\\\\wsl$\\DistroName\\\\home\\username\\path\\test.png";
        let src_path = "";// your director here
        return copy_to_wsl_clipboard(src_path);
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn check_if_folder_exist() {
        if !wsl::is_wsl() {
            return;
        }

        let os_linux_release = sys_info::linux_os_release().unwrap();
        let wsl_path = format!(
            "\\\\wsl$\\{}",
            os_linux_release.name()
        );

        println!("path {}", wsl_path);

        let powershell = Command::new("/mnt/c/Windows//System32/WindowsPowerShell/v1.0/powershell.exe")
        .arg("-NoProfile")
        .arg("-Command")
        .arg(&format!("Test-Path -path \"{}\"", wsl_path))
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output();

        let stdout = String::from_utf8(powershell.unwrap().stdout).unwrap();

        println!("output {:?}", stdout);
        println!("exist {}", stdout == "True\r\n");
        println!("does not exist {}", stdout == "False\r\n");
    }
}

