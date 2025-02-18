use std::{
    ffi::{c_void, CString},
    mem::{self, size_of},
};

use gl::{
    types::{GLchar, GLint, GLuint, GLvoid},
    ARRAY_BUFFER, COLOR_BUFFER_BIT, COMPILE_STATUS, FRAGMENT_SHADER, INFO_LOG_LENGTH, LINK_STATUS,
    STATIC_DRAW, VERTEX_SHADER,
};
use glfw::{Action, Context};

const VERTEX_SHADER_SOURCE: &str = r#"
#version 330 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;

out vec3 ourColor;

void main()
{
    gl_Position = vec4(aPos, 1.0);
    ourColor = aColor;
}
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
#version 330 core

out vec4 FragColor;
in vec3 ourColor;

void main()
{
    FragColor = vec4(ourColor, 1.0f);
    
} 
"#;

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, _events) = glfw
        .create_window(800, 600, "LearnOpenGL", glfw::WindowMode::Windowed)
        .unwrap();

    gl::load_with(|s| window.get_proc_address(s) as *const std::os::raw::c_void);

    window.make_current();
    window.set_framebuffer_size_polling(true);

    let vertex_shader: u32;
    unsafe {
        vertex_shader = gl::CreateShader(VERTEX_SHADER);
        gl::ShaderSource(
            vertex_shader,
            1,
            &CString::new(VERTEX_SHADER_SOURCE).unwrap().as_ptr(),
            std::ptr::null(),
        );
        gl::CompileShader(vertex_shader);

        let mut success = gl::FALSE as GLint;
        gl::GetShaderiv(vertex_shader, COMPILE_STATUS, &mut success);

        if success != gl::TRUE as GLint {
            let mut len = 0;
            gl::GetShaderiv(vertex_shader, INFO_LOG_LENGTH, &mut len);

            let mut buffer: Vec<u8> = vec![0; len as usize];
            gl::GetShaderInfoLog(
                vertex_shader,
                len,
                std::ptr::null_mut(),
                buffer.as_mut_ptr() as *mut GLchar,
            );
            eprintln!(
                "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}",
                String::from_utf8_lossy(&buffer)
            );
        }
    };

    let fragment_shader: u32;
    unsafe {
        fragment_shader = gl::CreateShader(FRAGMENT_SHADER);
        gl::ShaderSource(
            fragment_shader,
            1,
            &CString::new(FRAGMENT_SHADER_SOURCE).unwrap().as_ptr(),
            std::ptr::null(),
        );
        gl::CompileShader(fragment_shader);

        let mut success = gl::FALSE as GLint;
        gl::GetShaderiv(fragment_shader, COMPILE_STATUS, &mut success);

        if success != gl::TRUE as GLint {
            let mut len = 0;
            gl::GetShaderiv(fragment_shader, INFO_LOG_LENGTH, &mut len);

            let mut buffer: Vec<u8> = vec![0; len as usize];
            gl::GetShaderInfoLog(
                fragment_shader,
                len,
                std::ptr::null_mut(),
                buffer.as_mut_ptr() as *mut GLchar,
            );
            eprintln!(
                "ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}",
                String::from_utf8_lossy(&buffer)
            );
        }
    }

    let shader_program: u32;
    unsafe {
        shader_program = gl::CreateProgram();

        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        let mut success = gl::FALSE as GLint;
        gl::GetProgramiv(shader_program, LINK_STATUS, &mut success);

        if success != gl::TRUE as GLint {
            let mut len = 0;
            gl::GetProgramiv(shader_program, INFO_LOG_LENGTH, &mut len);

            let mut buffer: Vec<u8> = vec![0; len as usize];
            gl::GetProgramInfoLog(
                shader_program,
                len,
                std::ptr::null_mut(),
                buffer.as_mut_ptr() as *mut GLchar,
            );
            eprintln!(
                "ERROR::SHADER::PROGRAM::LINKING_FAILED\n{}",
                String::from_utf8_lossy(&buffer)
            );
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }

    #[rustfmt::skip]
    let vertices: [f32; 18] = [
        // positions        // colors
        -0.5, -0.5, 0.0,    1.0, 0.0, 0.0,
         0.5, -0.5, 0.0,    0.0, 1.0, 0.0,
         0.0,  0.5, 0.0,    0.0, 0.0, 1.0
    ];

    let mut vao: GLuint = 0;
    let mut vbo: GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(ARRAY_BUFFER, vbo);
        gl::BufferData(
            ARRAY_BUFFER,
            mem::size_of_val(&vertices) as _,
            vertices.as_ptr() as *const GLvoid,
            STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            6 * size_of::<f32>() as i32,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            6 * size_of::<f32>() as i32,
            (3 * size_of::<f32>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(1);
    };

    while !window.should_close() {
        process_input(&mut window);

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(COLOR_BUFFER_BIT);

            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        };

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_input(window: &mut glfw::Window) {
    if window.get_key(glfw::Key::Escape) == Action::Press {
        window.set_should_close(true);
    }
}
