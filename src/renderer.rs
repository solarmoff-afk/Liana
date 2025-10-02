use std::ffi::c_void;
use std::mem;
use std::ptr;
use glam::Mat4;
use crate::{LianaShader, LianaState};
use crate::objects2d::{InstanceData, QuadVertex};

pub const MAX_INSTANCES: usize = 10000;

pub struct RenderState {
    pub vao: u32,
    pub vbo_quad: u32,
    pub vbo_instances: u32,
    pub rect_shader: LianaShader,
    pub(crate) instances: Vec<InstanceData>,
}

impl RenderState {
    pub fn new() -> Result<Self, String> {
        let vertex_shader_source = r#"
            #version 330 core
            layout (location = 0) in vec2 aLocalPos;
            layout (location = 1) in vec3 aWorldPos;
            layout (location = 2) in vec4 aColor;
            layout (location = 3) in vec2 aRectSize;
            layout (location = 4) in vec4 aRadii;

            out vec4 vColor;
            out vec2 vLocalPos;
            out vec2 vRectSize;
            out vec4 vRadii;

            uniform mat4 view;
            uniform mat4 projection;

            void main() {
                vec2 finalPos = aWorldPos.xy + aLocalPos * aRectSize;
                gl_Position = projection * view * vec4(finalPos, aWorldPos.z, 1.0);
                vColor = aColor;
                vRectSize = aRectSize;
                vLocalPos = aLocalPos * aRectSize;
                vRadii = aRadii;
            }
        "#;

        let fragment_shader_source = r#"
            #version 330 core
            out vec4 FragColor;

            in vec4 vColor;
            in vec2 vLocalPos;
            in vec2 vRectSize;
            in vec4 vRadii;

            float sdRoundedBox(in vec2 p, in vec2 b, in vec4 r) {
                r.xy = (p.x > 0.0) ? r.xy : r.wz;
                r.x = (p.y > 0.0) ? r.x : r.y;
                vec2 q = abs(p) - b + r.x;
                return min(max(q.x, q.y), 0.0) + length(max(q, 0.0)) - r.x;
            }

            void main() {
                vec2 halfSize = vRectSize * 0.5;
                vec2 p = vLocalPos - halfSize;

                float minHalfDim = min(halfSize.x, halfSize.y);
                vec4 clampedRadii = min(vRadii, vec4(minHalfDim));

                float dist = sdRoundedBox(p, halfSize, clampedRadii);
                float alpha = 1.0 - smoothstep(0.0, 2.0, dist);

                if (alpha < 0.01) {
                    discard;
                }
                FragColor = vec4(vColor.rgb, vColor.a * alpha);
            }
        "#;

        let rect_shader = LianaShader::new(vertex_shader_source, fragment_shader_source)?;

        let mut vao = 0;
        let mut vbo_quad = 0;
        let mut vbo_instances = 0;

        unsafe {
            let quad_vertices: [QuadVertex; 4] = [
                QuadVertex { local_pos: [0.0, 0.0] },
                QuadVertex { local_pos: [1.0, 0.0] },
                QuadVertex { local_pos: [0.0, 1.0] },
                QuadVertex { local_pos: [1.0, 1.0] },
            ];

            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo_quad);
            gl::GenBuffers(1, &mut vbo_instances);
            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo_quad);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                mem::size_of_val(&quad_vertices) as isize,
                quad_vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, mem::size_of::<QuadVertex>() as i32, ptr::null());

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo_instances);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (MAX_INSTANCES * mem::size_of::<InstanceData>()) as isize,
                ptr::null(),
                gl::DYNAMIC_DRAW,
            );

            let stride = mem::size_of::<InstanceData>() as i32;
            let mut offset = 0;

            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, offset as *const _);
            offset += mem::size_of::<[f32; 3]>();

            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(2, 4, gl::FLOAT, gl::FALSE, stride, offset as *const _);
            offset += mem::size_of::<[f32; 4]>();
            
            gl::EnableVertexAttribArray(3);
            gl::VertexAttribPointer(3, 2, gl::FLOAT, gl::FALSE, stride, offset as *const _);
            offset += mem::size_of::<[f32; 2]>();
            
            gl::EnableVertexAttribArray(4);
            gl::VertexAttribPointer(4, 4, gl::FLOAT, gl::FALSE, stride, offset as *const _);

            gl::VertexAttribDivisor(1, 1);
            gl::VertexAttribDivisor(2, 1);
            gl::VertexAttribDivisor(3, 1);
            gl::VertexAttribDivisor(4, 1);

            gl::BindVertexArray(0);
        }

        Ok(Self {
            vao, vbo_quad, vbo_instances, rect_shader,
            instances: Vec::with_capacity(MAX_INSTANCES),
        })
    }

    pub fn flush(&mut self, state: &LianaState) {
        if self.instances.is_empty() { return; }

        self.rect_shader.use_program();
        
        unsafe {
            gl::UniformMatrix4fv(self.rect_shader.view_loc, 1, gl::FALSE, &state.view.to_cols_array()[0]);
            gl::UniformMatrix4fv(self.rect_shader.proj_loc, 1, gl::FALSE, &state.projection.to_cols_array()[0]);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo_instances);
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (self.instances.len() * mem::size_of::<InstanceData>()) as isize,
                self.instances.as_ptr() as *const _,
            );

            gl::BindVertexArray(self.vao);
            gl::DrawArraysInstanced(gl::TRIANGLE_STRIP, 0, 4, self.instances.len() as i32);
            gl::BindVertexArray(0);
        }

        self.instances.clear();
    }
}