use std::ffi::{CString};
use std::ptr;
use std::str;

pub struct LianaShader {
    pub program_id: u32,
    pub view_loc: i32,
    pub proj_loc: i32,
}

impl LianaShader {
    pub fn new(vertex_shader_source: &str, fragment_shader_source: &str) -> Result<Self, String> {
        unsafe {
            let vertex_shader = compile_shader(vertex_shader_source, gl::VERTEX_SHADER)?;
            let fragment_shader = compile_shader(fragment_shader_source, gl::FRAGMENT_SHADER)?;

            let program_id = gl::CreateProgram();
            gl::AttachShader(program_id, vertex_shader);
            gl::AttachShader(program_id, fragment_shader);
            gl::LinkProgram(program_id);

            let mut success = 0;
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);

            if success == 0 {
                let mut len = 0;
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
                
                let mut info_log = Vec::with_capacity(len as usize);
                info_log.set_len((len as usize).saturating_sub(1));
                
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut gl::types::GLchar,
                );

                return Err(str::from_utf8(&info_log)
                    .unwrap_or("Linker error log was not utf8")
                    .to_string());
            }

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            let view_loc = gl::GetUniformLocation(program_id, CString::new("view").unwrap().as_ptr());
            let proj_loc = gl::GetUniformLocation(program_id, CString::new("projection").unwrap().as_ptr());

            Ok(LianaShader {
                program_id,
                view_loc, proj_loc,
            })
        }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.program_id);
        }
    }
}

fn compile_shader(source: &str, shader_type: gl::types::GLenum) -> Result<u32, String> {
    unsafe {
        let shader = gl::CreateShader(shader_type);
        let c_source = CString::new(source.as_bytes()).unwrap();

        gl::ShaderSource(shader, 1, &c_source.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        let mut success = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

        if success == 0 {
            let mut len = 0;
            
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut info_log = Vec::with_capacity(len as usize);
            
            info_log.set_len((len as usize).saturating_sub(1));
            
            gl::GetShaderInfoLog(
                shader,
                len,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut gl::types::GLchar,
            );

            return Err(str::from_utf8(&info_log)
                .unwrap_or("Shader error log was not utf8")
                .to_string());
        }

        Ok(shader)
    }
}