use MFEKMath::{Piecewise, Evaluate, Vector, VWSContour, VWSSettings, VWSHandle, variable_width_stroke, parse_vws_lib, JoinType, CapType};
use MFEKMath::variable_width_stroking::{InterpolationType, generate_vws_lib};
use super::prelude::*;
use crate::state::Follow;
use glifparser::{Handle, WhichHandle};
use skulpin::skia_safe::{
    Canvas, ContourMeasureIter, Font, FontStyle, Matrix, Paint, PaintStyle, Path, Point, Rect,
    TextBlob, Typeface,
};

pub fn on_load_glif()
{
    STATE.with(|v| {

        let mut _v = v.borrow_mut();

        if let Some(vws_contours) = parse_vws_lib(&_v.glyph.as_ref().unwrap().glif)
        {
            println!("herp");
            _v.vws_contours = vws_contours.0;
            _v.glyph.as_mut().unwrap().glif.lib = Some(vws_contours.1);
        }
    });

    STATE.with(|v| {
        generate_previews(v)
    })

}

pub fn generate_lib(vwscontours: Vec<VWSContour>) -> Option<xmltree::Element>
{
    return generate_vws_lib(&vwscontours)
}

fn get_vws_contour(v: &RefCell<state::State<Option<state::PointData>>>, contour_idx: usize) -> Option<usize>
{
    for (idx, vwscontour) in v.borrow().vws_contours.iter().enumerate() {
        if vwscontour.id == contour_idx {
            return Some(idx);
        }
    }

    None
}

fn fix_vws_contour(v: &RefCell<state::State<Option<state::PointData>>>, contour_idx: usize) 
{
    let contour_size = get_outline!(v)[contour_idx].len();
    let vws_contour_size = v.borrow().vws_contours[contour_idx].handles.len();

    let difference = vws_contour_size - (contour_size + 1);
    if difference != 0
    {
        for i in 0 .. difference {
            v.borrow_mut().vws_contours[contour_idx].handles.push(VWSHandle{
                left_offset: 10.,
                right_offset: 10.,
                interpolation: InterpolationType::Linear
            })
        }
    }
}

fn generate_vws_contour(v: &RefCell<state::State<Option<state::PointData>>>, contour_idx: usize)
{
    let mut new_vws_contour = VWSContour {
        handles: Vec::new(),
        id: contour_idx
    };

    for i in 0.. get_outline!(v)[contour_idx].len() + 1 {
        new_vws_contour.handles.push(VWSHandle{
            left_offset: 10.,
            right_offset: 10.,
            interpolation: InterpolationType::Linear
        })
    }

    v.borrow_mut().vws_contours.push(new_vws_contour);
}

fn get_vws_handle(v: &RefCell<state::State<Option<state::PointData>>>, vcontour: Option<usize>, handle_idx: usize) -> VWSHandle
{
    if let Some(vc) = vcontour {
        // if the contour exists but this handle doesn't we're gonna add handles until we've got
        // the right amount
        if v.borrow().vws_contours[vc].handles.len() < handle_idx
        {
            fix_vws_contour(v, vc);
        }

        return v.borrow().vws_contours[vc].handles[handle_idx].clone();
    }

    return VWSHandle {
        left_offset: 10.,
        right_offset: 10.,
        interpolation: InterpolationType::Linear,
    }
}

fn set_vws_handle(v: &RefCell<state::State<Option<state::PointData>>>, contour_idx: usize, handle_idx: usize, side: WhichHandle, pos: f64)
{
    if get_vws_contour(v, contour_idx).is_none() {
        generate_vws_contour(v, contour_idx);
    }

    // we know this contour exists now
    let vws_contour = get_vws_contour(v, contour_idx).unwrap();

    let id = v.borrow().vws_contours[vws_contour].id;
    let contour_pw =  Piecewise::from(&get_outline!(v)[id]);

    if handle_idx == 0 && contour_pw.is_closed() {
        let last_handle = v.borrow().vws_contours[vws_contour].handles.len() - 1;
        match side {
            WhichHandle::A => v.borrow_mut().vws_contours[vws_contour].handles[last_handle].left_offset = pos,
            WhichHandle::B => v.borrow_mut().vws_contours[vws_contour].handles[last_handle].right_offset = pos,
            _ => {} // should be unreachable
        }
    }

    match side {
        WhichHandle::A => v.borrow_mut().vws_contours[vws_contour].handles[handle_idx].left_offset = pos,
        WhichHandle::B => v.borrow_mut().vws_contours[vws_contour].handles[handle_idx].right_offset = pos,
        _ => {} // should be unreachable
    }

}

fn set_all_vws_handles(v: &RefCell<state::State<Option<state::PointData>>>, contour_idx: usize, handle_idx: usize, side: WhichHandle, pos: f64)
{
    if get_vws_contour(v, contour_idx).is_none() {
        generate_vws_contour(v, contour_idx);
    }

    STATE.with(|v| {// we know this contour exists now
        let vws_contour = get_vws_contour(v, contour_idx).unwrap();

        let mut borrowed_v = v.borrow_mut();
        for handle_idx in 0 .. borrowed_v.vws_contours[vws_contour].handles.len() {
            let ctrl = TOOL_DATA.with(|v| v.borrow().ctrl);

            if ctrl {
                borrowed_v.vws_contours[vws_contour].handles[handle_idx].left_offset = pos;
                borrowed_v.vws_contours[vws_contour].handles[handle_idx].right_offset = pos;
            } else {
                match side {
                    WhichHandle::A => borrowed_v.vws_contours[vws_contour].handles[handle_idx].left_offset = pos,
                    WhichHandle::B => borrowed_v.vws_contours[vws_contour].handles[handle_idx].right_offset = pos,
                    _ => {} // should be unreachable
                }
            }
        }
    });
}

fn generate_previews(v: &RefCell<state::State<Option<state::PointData>>>)
{
    let mut new_previews = Vec::new();

    for vws_contour in &v.borrow().vws_contours {
        let contour_pw = Piecewise::from(&get_outline!(v)[vws_contour.id]);

        let settings = VWSSettings {
            join_type: JoinType::Round,
            cap_type_start: CapType::Round,
            cap_type_end: CapType::Round,
            cap_custom_start: None,
            cap_custom_end: None
        };

        use std::time::Instant;
        let t = Instant::now();
        let vws_output = variable_width_stroke(&contour_pw, &vws_contour.handles, &settings);
        println!("{:.2}", t.elapsed().as_millis());

        for contour in vws_output.segs {
            new_previews.push(contour.to_contour());
        }
    }

    v.borrow_mut().vws_previews = Some(new_previews);
}

fn mouse_coords_to_handle_space(
    v: &RefCell<state::State<Option<state::PointData>>>, contour_idx: usize, handle_idx: usize, side: WhichHandle, mousepos: Vector
) -> f64
{
    let (start_pos, handle_pos) = get_vws_handle_pos(v, contour_idx, handle_idx, side);

    return mousepos.distance(start_pos);
}

// this function is more expensive than it lets on
fn get_vws_handle_pos(v: &RefCell<state::State<Option<state::PointData>>>, contour_idx: usize, handle_idx: usize, side: WhichHandle) -> (Vector, Vector)
{
    let vws_contour = get_vws_contour(v, contour_idx);
    let contour_pw = Piecewise::from(&get_outline!(v)[contour_idx]);

    if handle_idx < contour_pw.segs.len()
    {
        let vws_handle = get_vws_handle(v, vws_contour, handle_idx);
        let bezier = &contour_pw.segs[handle_idx];
        let start_point = bezier.start_point();
        let tangent = bezier.tangent_at(0.);
        let normal = Vector{x: tangent.y, y: -tangent.x}.normalize();

        match side {
            WhichHandle::A => return (start_point, start_point + normal * vws_handle.left_offset),
            WhichHandle::B => return (start_point, start_point + normal * vws_handle.left_offset),
            _ => panic!("Should be unreachable!")
        }
    }
    else
    {
        let vws_handle = get_vws_handle(v, vws_contour, handle_idx);
        let bezier = &contour_pw.segs.last().unwrap();
        let start_point = bezier.end_point();
        let tangent = bezier.tangent_at(1.);
        let normal = Vector{x: tangent.y, y: -tangent.x}.normalize();

        match side {
            WhichHandle::A => return (start_point, start_point + normal * vws_handle.left_offset),
            WhichHandle::B => return (start_point, start_point + normal * vws_handle.left_offset),
            _ => panic!("Should be unreachable!")
        }
    }
}

fn vws_clicked_point_or_handle(
    position: PhysicalPosition<f64>,
    v: &RefCell<state::State<Option<state::PointData>>>,
) -> Option<(usize, usize, WhichHandle)> {
    let factor = v.borrow().factor;
    let mposition = update_mousepos(position, &v, true);
    let _contour_idx = 0;
    let _point_idx = 0;

    for (contour_idx, contour) in get_outline!(v).iter().enumerate() {
        let contour_pw = Piecewise::from(contour);

        let vws_contour = get_vws_contour(v, contour_idx);

        let size = ((POINT_RADIUS * 2.) + (POINT_STROKE_THICKNESS * 2.)) * (1. / factor);
        for (vws_handle_idx, bezier) in contour_pw.segs.iter().enumerate() {

            let start_point = bezier.start_point();
            let tangent = bezier.tangent_at(0.);
            let normal = Vector{x: tangent.y, y: -tangent.x}.normalize();

            let vws_handle = get_vws_handle(v, vws_contour, vws_handle_idx);

            let handle_pos_left = start_point + normal * vws_handle.left_offset;
            let handle_pos_right = start_point + normal * -vws_handle.right_offset;

            let handle_left_point = SkPoint::new(
                calc_x(handle_pos_left.x as f32) - (size / 2.),
                calc_y(handle_pos_left.y as f32) - (size / 2.),
            );
            let handle_left_rect = SkRect::from_point_and_size(handle_left_point, (size, size));

            let handle_right_point = SkPoint::new(
                calc_x(handle_pos_right.x as f32) - (size / 2.),
                calc_y(handle_pos_right.y as f32) - (size / 2.),
            );
            let handle_right_rect = SkRect::from_point_and_size(handle_right_point, (size, size));

            let sk_mpos = SkPoint::new(mposition.x as f32, mposition.y as f32);

            if handle_left_rect.contains(sk_mpos) {
                return Some((contour_idx, vws_handle_idx, WhichHandle::A));
            }
            else if handle_right_rect.contains(sk_mpos)
            {
                return Some((contour_idx, vws_handle_idx, WhichHandle::B));
            }
        }


    }

    None
}

pub fn mouse_pressed(
    position: PhysicalPosition<f64>,
    v: &RefCell<state::State<Option<state::PointData>>>,
    meta: MouseMeta,
) -> bool {
    match vws_clicked_point_or_handle(position, v) {
        Some((ci, pi, wh)) => TOOL_DATA.with(|p| {
            let follow: Follow = meta.into();
            debug!(
                "Clicked point: {:?} {:?}. Follow behavior: {}",
                get_outline!(v)[ci][pi],
                wh,
                follow
            );
            p.borrow_mut().contour = Some(ci);
            p.borrow_mut().cur_point = Some(pi);
            p.borrow_mut().follow = follow;
            p.borrow_mut().handle = wh;
            p.borrow_mut().shift = meta.modifiers.shift();
            p.borrow_mut().ctrl = meta.modifiers.ctrl();

            true
        }),
        None => TOOL_DATA.with(|p| {
            p.borrow_mut().contour = None;
            p.borrow_mut().cur_point = None;
            p.borrow_mut().handle = WhichHandle::Neither;
            false
        }),
    };

    false
}

// Placeholder
pub fn mouse_button<T>(
    _position: PhysicalPosition<f64>,
    _v: &RefCell<state::State<T>>,
    _meta: MouseMeta,
) -> bool {
    false
}

pub fn mouse_released(
    _position: PhysicalPosition<f64>,
    v: &RefCell<state::State<Option<state::PointData>>>,
    _meta: MouseMeta,
) -> bool {
    TOOL_DATA.with(|p| {
        p.borrow_mut().contour = None;
        p.borrow_mut().cur_point = None;
        p.borrow_mut().handle = WhichHandle::Neither;
        true
    })
}

/// Get indexes stored by clicked_point_or_handle and move the points they refer to around.
pub fn mouse_moved(position: PhysicalPosition<f64>, v: &RefCell<state::State<Option<state::PointData>>>) -> bool {
    let mposition = update_mousepos(position, &v, false);
    if !v.borrow().mousedown {
        return false;
    }

    let x = calc_x(mposition.x as f32);
    let y = calc_y(mposition.y as f32);
    let contour = TOOL_DATA.with(|p| p.borrow().contour);
    let cur_point = TOOL_DATA.with(|p| p.borrow().cur_point);
    let which_handle = TOOL_DATA.with(|p| p.borrow().handle);
    let shift = TOOL_DATA.with(|p| p.borrow().shift);
    let ctrl = TOOL_DATA.with(|p| p.borrow().ctrl);

    match (contour, cur_point, which_handle) {
        // A control point (A or B) is being moved.
        (Some(ci), Some(pi), wh) => {
            let new_pos = mouse_coords_to_handle_space(v, ci, pi, wh, Vector{x:x as f64, y:y as f64});
            // if shift is held down we scale all the points
            if shift || ctrl{
                set_all_vws_handles(v, ci, pi, wh, new_pos);
            }
            else
            {
                set_vws_handle(v, ci, pi, wh, new_pos);
            }

            generate_previews(v);
            false
        }
        _ => false,
    };

    true
}

pub fn update_previews(position: PhysicalPosition<f64>, v: &RefCell<state::State<Option<state::PointData>>>) -> bool {
    let mposition = update_mousepos(position, &v, false);
    if !v.borrow().mousedown {
        return false;
    }
    generate_previews(v);

    true
}

pub fn should_draw_contour(v: &RefCell<state::State<Option<state::PointData>>>, idx: usize) -> bool
{
    if get_vws_contour(v, idx).is_some()
    {
        return false;
    }
    
    return true;
}

pub fn draw_handles(canvas: &mut Canvas) {
    STATE.with(|v| {
        let factor = v.borrow().factor;

        for (contour_idx, contour) in get_outline!(v).iter().enumerate() {
            let contour_pw = Piecewise::from(contour);
    
            let vws_contour = get_vws_contour(v, contour_idx);
    
            let size = ((POINT_RADIUS * 2.) + (POINT_STROKE_THICKNESS * 2.)) * (1. / factor);
            let last_handle = 0;
            for (vws_handle_idx, bezier) in contour_pw.segs.iter().enumerate() {
    
                let start_point = bezier.start_point();
                let tangent = bezier.tangent_at(0.);
                let normal = Vector{x: tangent.y, y: -tangent.x}.normalize();
    
                let vws_handle = get_vws_handle(v, vws_contour, vws_handle_idx);
    
                let handle_pos_left = start_point + normal * vws_handle.left_offset;
                let handle_pos_right = start_point + normal * -vws_handle.right_offset;
    
                let mut path = Path::new();
                let mut paint = Paint::default();

                paint.set_anti_alias(true);
                paint.set_color(HANDLEBAR_STROKE);
                paint.set_stroke_width(HANDLEBAR_THICKNESS * (1. / factor));
                paint.set_style(PaintStyle::Stroke);

                path.move_to((calc_x(handle_pos_left.x as f32), calc_y(handle_pos_left.y as f32)));
                path.line_to((calc_x(start_point.x as f32 ), calc_y(start_point.y as f32)));
                path.line_to((calc_x(handle_pos_right.x as f32), calc_y(handle_pos_right.y as f32)));

                canvas.draw_path(&path, &paint);
            } 
        }
    })
}
