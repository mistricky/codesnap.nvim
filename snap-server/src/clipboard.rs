use arboard::Clipboard;
use arboard::ImageData;
use image::load_from_memory;

pub fn copy_base64_image_into_clipboard(base64_image: String) {
    copy_memory_image_into_clipboard(&image_base64::from_base64(base64_image))
}

pub fn copy_memory_image_into_clipboard(buffer: &Vec<u8>) {
    let dynamic_image = load_from_memory(buffer).unwrap();

    let image = ImageData {
        width: dynamic_image.width() as usize,
        height: dynamic_image.height() as usize,
        bytes: dynamic_image.as_bytes().into(),
    };

    let mut clipboard = Clipboard::new().unwrap();

    clipboard.set_image(image).unwrap();
}
