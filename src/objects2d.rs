use crate::lifecycle::RENDER_STATE;
use crate::renderer::{RenderState, MAX_INSTANCES};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct QuadVertex {
    pub local_pos: [f32; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct InstanceData {
    pub world_pos: [f32; 3],
    pub color: [f32; 4],
    pub rect_size: [f32; 2],
    pub radii: [f32; 4],
}

#[no_mangle]
pub unsafe extern "C" fn add_rect(
    x: f32, y: f32, z: f32, width: f32, height: f32, 
    r: f32, g: f32, b: f32, a: f32,
    r_tl: f32, r_tr: f32, r_br: f32, r_bl: f32
) {
    let render_state = RENDER_STATE.assume_init_mut();
    
    if render_state.instances.len() >= MAX_INSTANCES {
        return;
    }

    render_state.instances.push(InstanceData {
        world_pos: [x, y, z],
        color: [r, g, b, a],
        rect_size: [width, height],
        radii: [r_tl, r_tr, r_br, r_bl],
    });
}