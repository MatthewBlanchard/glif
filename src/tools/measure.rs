use glifrenderer::constants::{self, OUTLINE_STROKE_THICKNESS};
use glifrenderer::string::UiString;
use skia_safe::{dash_path_effect, Canvas, Paint, Path, Point};
use MFEKmath::Vector;

use crate::editor::Editor;
use crate::user_interface::Interface;

use super::prelude::*;

#[derive(Clone, Debug)]
pub struct Measure {
    measure_from: Option<(f32, f32)>,
}

impl Tool for Measure {
    #[rustfmt::skip]
    fn event(&mut self, v: &mut Editor, _i: &mut Interface, event: EditorEvent) {
        if let EditorEvent::MouseEvent { mouse_info, event_type } = event {
            match event_type {
                MouseEventType::Pressed => self.mouse_pressed(v, mouse_info),
                MouseEventType::Released => self.mouse_released(v, mouse_info),
                _ => (),
            }
        }
    }

    fn draw(&mut self, _v: &Editor, i: &Interface, canvas: &mut Canvas) {
        self.draw_line(i, canvas);
    }
}

impl Measure {
    pub fn new() -> Self {
        Self { measure_from: None }
    }

    fn mouse_pressed(&mut self, _v: &Editor, mouse_info: MouseInfo) {
        self.measure_from = Some(mouse_info.position);
    }

    fn mouse_released(&mut self, _v: &Editor, _mouse_info: MouseInfo) {
        self.measure_from = None;
    }

    fn draw_line(&self, i: &Interface, canvas: &mut Canvas) {
        let mut path = Path::new();
        let mut paint = Paint::default();
        let factor = i.viewport.factor;

        if let Some(measure_from) = self.measure_from {
            let skpath_start = Point::new(measure_from.0 as f32, measure_from.1 as f32);
            let skpath_end = Point::new(
                i.mouse_info.position.0 as f32,
                i.mouse_info.position.1 as f32,
            );

            let start_vec = Vector::from_skia_point(&skpath_start);
            let end_vec = Vector::from_skia_point(&skpath_end);
            let halfway = start_vec.lerp(end_vec, 0.5);
            let unit_vec = (end_vec - start_vec).normalize();
            let angle = f64::atan2(unit_vec.y, unit_vec.x);
            let distance = start_vec.distance(end_vec) * (1. / factor) as f64;

            path.move_to(skpath_start);
            path.line_to(skpath_end);
            paint.set_color(constants::MEASURE_STROKE);
            paint.set_style(skia_safe::PaintStyle::Stroke);
            paint.set_stroke_width(OUTLINE_STROKE_THICKNESS * (1. / factor));
            let dash_offset = (1. / factor) * 5.;
            paint.set_path_effect(dash_path_effect::new(&[dash_offset, dash_offset], 0.0));
            canvas.draw_path(&path, &paint);

            draw_measure_string(
                i,
                (halfway.x as f32, halfway.y as f32),
                angle as f32,
                format! {"{0:.3}", distance}.as_str(),
                canvas,
            );
        }
    }
}

pub fn draw_measure_string(
    i: &Interface,
    at: (f32, f32),
    angle: f32,
    s: &str,
    canvas: &mut Canvas,
) {
    let s = UiString::centered_with_colors(s, MEASURE_STROKE, None).rotated(angle.to_degrees());

    s.draw(&i.viewport, at, canvas);
}
