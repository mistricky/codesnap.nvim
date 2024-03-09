use cosmic_text::{Attrs, Buffer, Color, FontSystem, Metrics, Shaping, SwashCache};
use tiny_skia::{Paint, Pixmap, Rect, Transform};

pub fn render_rich_text(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    font_size: f32,
    line_height: f32,
    scale_factor: f32,
    spans: Vec<(&str, Attrs)>,
    default_font_color: Color,
    pixmap: &mut Pixmap,
) {
    let mut font_system = FontSystem::new();
    let mut swash_cache = SwashCache::new();
    let metrics = Metrics::new(font_size, line_height).scale(scale_factor.clone());
    let mut buffer = Buffer::new(&mut font_system, metrics);
    let mut buffer = buffer.borrow_with(&mut font_system);
    let attrs = Attrs::new();

    buffer.set_size(w * scale_factor, h * scale_factor);
    buffer.set_rich_text(spans, attrs, Shaping::Advanced);
    buffer.draw(
        &mut swash_cache,
        default_font_color,
        |font_x, font_y, w, h, color| {
            let mut paint = Paint {
                anti_alias: true,
                ..Default::default()
            };

            paint.set_color_rgba8(color.r(), color.g(), color.b(), color.a());
            let rect = Rect::from_xywh(
                font_x as f32 + x * scale_factor,
                font_y as f32 + y * scale_factor,
                w as f32,
                h as f32,
            )
            .unwrap();
            pixmap.fill_rect(rect, &paint, Transform::identity(), None);
        },
    );
}
