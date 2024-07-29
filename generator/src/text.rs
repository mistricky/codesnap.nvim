use cosmic_text::{
    Align, Attrs, AttrsList, Buffer, BufferLine, Color, FontSystem, LineEnding, Metrics, Shaping,
    SwashCache,
};
use tiny_skia::{Paint, Pixmap, Rect, Transform};

pub struct FontRenderer {
    font_system: FontSystem,
    scale_factor: f32,
    metrics: Metrics,
}

impl FontRenderer {
    pub fn new(
        font_size: f32,
        line_height: f32,
        scale_factor: f32,
        fonts_folder: &str,
    ) -> FontRenderer {
        let mut font_system = FontSystem::new();

        font_system.db_mut().load_fonts_dir(fonts_folder);

        let metrics = Metrics::new(font_size, line_height).scale(scale_factor.clone());

        FontRenderer {
            metrics,
            font_system,
            scale_factor,
        }
    }

    pub fn draw_text(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        spans: Vec<(&str, Attrs)>,
        pixmap: &mut Pixmap,
    ) {
        let mut buffer = Buffer::new(&mut self.font_system, self.metrics);
        buffer.set_size(
            &mut self.font_system,
            Some(w * self.scale_factor),
            Some(h * self.scale_factor),
        );
        buffer.set_rich_text(
            &mut self.font_system,
            spans,
            Attrs::new(),
            Shaping::Advanced,
        );
        self.draw(x, y, &mut buffer, pixmap);
    }

    pub fn draw_line(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        line: &str,
        attrs: Attrs,
        align: Option<Align>,
        pixmap: &mut Pixmap,
    ) {
        let mut buffer = Buffer::new(&mut self.font_system, self.metrics);
        let mut line = if cfg!(unix) {
            BufferLine::new(
                line,
                LineEnding::Lf,
                AttrsList::new(attrs),
                Shaping::Advanced,
            )
        } else if cfg!(windows) {
            BufferLine::new(
                line,
                LineEnding::CrLf,
                AttrsList::new(attrs),
                Shaping::Advanced,
            )
        } else {
            panic!("Unsupported OS")
        };

        line.set_align(align);
        buffer.lines = vec![line];
        buffer.set_size(&mut self.font_system, Some(w), Some(h));
        self.draw(x, y, &mut buffer, pixmap);
    }

    fn draw<'a>(&mut self, x: f32, y: f32, buffer: &mut Buffer, pixmap: &mut Pixmap) {
        let mut swash_cache = SwashCache::new();
        let default_font_color = Color::rgb(255, 255, 255);

        buffer.draw(
            &mut self.font_system,
            &mut swash_cache,
            default_font_color,
            |font_x, font_y, w, h, color| {
                let mut paint = Paint {
                    anti_alias: true,
                    ..Default::default()
                };

                paint.set_color_rgba8(color.r(), color.g(), color.b(), color.a());

                let rect = Rect::from_xywh(
                    font_x as f32 + x * self.scale_factor,
                    font_y as f32 + y * self.scale_factor,
                    w as f32,
                    h as f32,
                )
                .expect("Cannot draw text on pixmap");

                pixmap.fill_rect(rect, &paint, Transform::identity(), None);
            },
        );
    }
}
