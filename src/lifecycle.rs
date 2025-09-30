use std::ffi::{c_void, CString};
use std::os::raw::c_char;
use std::sync::Once;

static INIT: Once = Once::new();
type GetProcAddress = unsafe extern "C" fn(*const c_char) -> *const c_void;

#[no_mangle]
pub unsafe extern "C" fn setupGL(get_proc_address: GetProcAddress) {
    INIT.call_once(|| {
        gl::load_with(|symbol| {
            let c_str = CString::new(symbol).unwrap();
            return get_proc_address(c_str.as_ptr());
        });
    });
}

#[no_mangle]
pub unsafe extern "C" fn render(r: f32, g: f32, b: f32) {
    gl::ClearColor(r, g, b, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT);
}