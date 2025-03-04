use std::{ffi::CString, fs, str::FromStr};

use gl::{
    types::{GLchar, GLint},
    COMPILE_STATUS, FALSE, FRAGMENT_SHADER, INFO_LOG_LENGTH, LINK_STATUS, VERTEX_SHADER,
};

pub struct Shader {
    pub id: u32,
}

#[allow(dead_code)]
impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Result<Self, ()> {
        let vertex_code = fs::read_to_string(vertex_path).unwrap_or(String::from(""));
        let fragment_code = fs::read_to_string(fragment_path).unwrap_or(String::from(""));

        if vertex_code == "" || fragment_code == "" {
            eprintln!("ERROR::SHADER::FILE_NOT_SUCCESFULLY_READ");
            return Err(());
        }

        let vertex: u32;
        let vertex_shader_source = CString::new(vertex_code.as_str()).unwrap();
        unsafe {
            vertex = gl::CreateShader(VERTEX_SHADER);
            gl::ShaderSource(vertex, 1, &vertex_shader_source.as_ptr(), std::ptr::null());
            gl::CompileShader(vertex);

            let mut success = gl::FALSE as GLint;
            gl::GetShaderiv(vertex, COMPILE_STATUS, &mut success);

            if success != gl::TRUE as GLint {
                let mut len = 0;
                gl::GetShaderiv(vertex, INFO_LOG_LENGTH, &mut len);

                let mut buffer: Vec<u8> = vec![0; len as usize];
                gl::GetShaderInfoLog(
                    vertex,
                    len,
                    std::ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut GLchar,
                );
                eprintln!(
                    "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}",
                    String::from_utf8_lossy(&buffer)
                );
            }
        }

        let fragment: u32;
        let fragment_shader_source = CString::new(fragment_code.as_str()).unwrap();
        unsafe {
            fragment = gl::CreateShader(FRAGMENT_SHADER);
            gl::ShaderSource(
                fragment,
                1,
                &fragment_shader_source.as_ptr(),
                std::ptr::null(),
            );
            gl::CompileShader(fragment);

            let mut success = gl::FALSE as GLint;
            gl::GetShaderiv(fragment, COMPILE_STATUS, &mut success);

            if success != gl::TRUE as GLint {
                let mut len = 0;
                gl::GetShaderiv(fragment, INFO_LOG_LENGTH, &mut len);

                let mut buffer: Vec<u8> = vec![0; len as usize];
                gl::GetShaderInfoLog(
                    fragment,
                    len,
                    std::ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut GLchar,
                );
                eprintln!(
                    "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}",
                    String::from_utf8_lossy(&buffer)
                );
            }
        }

        let id = unsafe { gl::CreateProgram() };

        unsafe {
            gl::AttachShader(id, vertex);
            gl::AttachShader(id, fragment);
            gl::LinkProgram(id);

            let mut success = gl::FALSE as GLint;
            gl::GetProgramiv(id, LINK_STATUS, &mut success);

            if success != gl::TRUE as GLint {
                let mut len = 0;
                gl::GetProgramiv(id, INFO_LOG_LENGTH, &mut len);

                let mut buffer: Vec<u8> = vec![0; len as usize];
                gl::GetProgramInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut GLchar,
                );
                eprintln!(
                    "ERROR::SHADER::PROGRAM::LINKING_FAILED\n{}",
                    String::from_utf8_lossy(&buffer)
                );
            }
        }

        unsafe {
            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment)
        };

        Ok(Self { id })
    }
    pub fn use_shader(&self) {
        unsafe { gl::UseProgram(self.id) }
    }

    pub fn set_bool(&self, name: &str, value: bool) {
        unsafe {
            gl::Uniform1i(
                gl::GetUniformLocation(self.id, CString::from_str(name).unwrap().as_ptr()),
                value.into(),
            );
        }
    }

    pub fn set_int(&self, name: &str, value: i32) {
        unsafe {
            gl::Uniform1i(
                gl::GetUniformLocation(self.id, CString::from_str(name).unwrap().as_ptr()),
                value,
            );
        }
    }

    pub fn set_float(&self, name: &str, value: f32) {
        unsafe {
            gl::Uniform1f(
                gl::GetUniformLocation(self.id, CString::from_str(name).unwrap().as_ptr()),
                value,
            )
        }
    }

    pub fn set_mat4(&self, name: &str, value: &nalgebra_glm::Mat4) {
        unsafe {
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.id, CString::from_str(name).unwrap().as_ptr()),
                1,
                FALSE,
                nalgebra_glm::value_ptr(value).as_ptr(),
            );
        }
    }
}
