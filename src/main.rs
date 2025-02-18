use std::{
    ffi::c_void,
    mem::{self, size_of},
};

use gl::{
    types::{GLuint, GLvoid},
    ARRAY_BUFFER, COLOR_BUFFER_BIT, STATIC_DRAW,
};
use glfw::{Action, Context};
use shader::Shader;

mod shader;

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

    let our_shader = Shader::new("shaders/vertex_shader.glsl", "shaders/fragment_shader.glsl").unwrap();

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

            our_shader.use_shader();
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
