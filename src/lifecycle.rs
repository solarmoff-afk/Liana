use std::ffi::{c_void, CString};
use std::mem::MaybeUninit;
use std::os::raw::c_char;
use std::sync::Once;
use glam::Mat4;
use crate::{LianaState, RenderState};

static INIT: Once = Once::new();
type GetProcAddress = unsafe extern "C" fn(*const c_char) -> *const c_void;

pub(crate) static mut RENDER_STATE: MaybeUninit<RenderState> = MaybeUninit::uninit();
pub(crate) static mut LIANA_STATE: MaybeUninit<LianaState> = MaybeUninit::uninit();

#[no_mangle]
pub unsafe extern "C" fn setupGL(get_proc_address: GetProcAddress) {
    INIT.call_once(|| {
        gl::load_with(|symbol| {
            let c_str = CString::new(symbol).unwrap();
            get_proc_address(c_str.as_ptr())
        });
    });
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    RENDER_STATE.write(RenderState::new().expect("Failed to initialize render state"));
    LIANA_STATE.write(LianaState::new());

    gl::Enable(gl::DEPTH_TEST);
    gl::Enable(gl::BLEND);
    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
}

#[no_mangle]
pub unsafe extern "C" fn render(width: i32, height: i32) {
    gl::ClearColor(0.1, 0.1, 0.1, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

    let liana_state = LIANA_STATE.assume_init_mut();
    liana_state.projection = Mat4::orthographic_rh_gl(0.0, width as f32, height as f32, 0.0, -1.0, 1.0);
    liana_state.view = Mat4::IDENTITY;

    let render_state = RENDER_STATE.assume_init_mut();
    render_state.flush(liana_state);
}

#[no_mangle]
pub unsafe extern "C" fn setViewport(width: i32, height: i32) {
    gl::Viewport(0, 0, width, height);
}