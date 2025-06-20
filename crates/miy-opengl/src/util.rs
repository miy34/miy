use crate as gl;

use alloc::{
    string::{String, ToString},
    vec,
};
use core::mem::MaybeUninit;

#[derive(Debug)]
pub enum ShaderError {
    FailedToCompile(String),
    FailedToLinkProgram(String),
}

fn create_shader(
    gl: &gl::Bindings,
    kind: gl::ShaderKind,
    source: &str,
) -> Result<gl::Shader, ShaderError> {
    unsafe {
        let shader = gl.create_shader(kind);
        gl.shader_source(
            shader,
            1,
            [source].as_ptr().cast(),
            [source.len()].as_ptr().cast(),
        );

        gl.compile_shader(shader);

        let mut success = 0;
        gl.get_shaderiv(
            shader,
            gl::ShaderParameterName::COMPILE_STATUS,
            &mut success,
        );

        let success = gl::Bool::from_int(success).expect("should be GL_TRUE or GL_FALSE");

        match success {
            gl::Bool::TRUE => Ok(shader),
            _ => {
                let mut error_log_length = 0;
                gl.get_shaderiv(
                    shader,
                    gl::ShaderParameterName::INFO_LOG_LENGTH,
                    &mut error_log_length,
                );

                let mut error = vec![b'\0'; error_log_length as usize];
                gl.get_shader_info_log(
                    shader,
                    error_log_length,
                    &mut error_log_length,
                    error.as_mut_ptr().cast(),
                );

                gl.delete_shader(shader);

                let error = alloc::ffi::CString::from_vec_with_nul(error)
                    .map_or_else(|e| e.to_string(), |e| e.to_string_lossy().to_string());

                Err(ShaderError::FailedToCompile(error))
            }
        }
    }
}

pub fn create_program_with_sources(
    gl: &gl::Bindings,
    vertex_source: &str,
    fragment_source: &str,
) -> Result<gl::Program, ShaderError> {
    let vertex_shader = create_shader(gl, gl::ShaderKind::VERTEX, vertex_source)?;
    let fragment_shader = create_shader(gl, gl::ShaderKind::FRAGMENT, fragment_source)?;

    unsafe {
        let program = gl.create_program();

        gl.attach_shader(program, vertex_shader);
        gl.attach_shader(program, fragment_shader);

        gl.link_program(program);

        gl.detach_shader(program, vertex_shader);
        gl.detach_shader(program, fragment_shader);

        gl.delete_shader(vertex_shader);
        gl.delete_shader(fragment_shader);

        Ok(program)
    }
}

pub fn create_buffer(
    gl: &gl::Bindings,
    target: gl::BufferTarget,
    usage: gl::BufferUsage,
    data: &[u8],
) -> gl::Buffer {
    unsafe {
        let mut buffer = MaybeUninit::uninit();
        gl.gen_buffers(1, buffer.as_mut_ptr());
        let buffer = buffer.assume_init();

        gl.bind_buffer(target, buffer);
        gl.buffer_data(
            target,
            data.len() as gl::Sizeiptr,
            data.as_ptr().cast(),
            usage,
        );
        buffer
    }
}
