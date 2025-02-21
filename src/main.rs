use std::{
    ffi::{c_int, c_void, CString},
    mem::{self, size_of},
    ptr::{null, null_mut},
};

use gl::{
    types::{GLint, GLuint, GLvoid},
    ARRAY_BUFFER, COLOR_BUFFER_BIT, ELEMENT_ARRAY_BUFFER, LINEAR, MIRRORED_REPEAT, NEAREST,
    STATIC_DRAW, TEXTURE_2D, TEXTURE_MAG_FILTER, TEXTURE_MIN_FILTER, TEXTURE_WRAP_S,
    TEXTURE_WRAP_T, UNSIGNED_BYTE, UNSIGNED_INT,
};
use glfw::{Action, Context};
use shader::Shader;
use stb_image::stb_image::{stbi_image_free, stbi_load};

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

    let our_shader =
        Shader::new("shaders/vertex_shader.glsl", "shaders/fragment_shader.glsl").unwrap();

    #[rustfmt::skip]
    let vertices: [f32; 32] = [
        // positions        // colors       // texture coords
         0.5,  0.5, 0.0,    1.0, 0.0, 0.0,  1.0, 1.0,
         0.5, -0.5, 0.0,    0.0, 1.0, 0.0,  1.0, 0.0,
        -0.5, -0.5, 0.0,    0.0, 0.0, 1.0,  0.0, 0.0,
        -0.5,  0.5, 0.0,    1.0, 1.0, 0.0,  0.0, 1.0
    ];

    #[rustfmt::skip]
    let indices: [u32; 6] = [
        0, 1, 3,
        1, 2, 3
    ];

    let mut vao: GLuint = 0;
    let mut vbo: GLuint = 0;
    let mut ebo: GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(ARRAY_BUFFER, vbo);
        gl::BufferData(
            ARRAY_BUFFER,
            mem::size_of_val(&vertices) as _,
            vertices.as_ptr() as *const GLvoid,
            STATIC_DRAW,
        );

        gl::BindBuffer(ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            ELEMENT_ARRAY_BUFFER,
            mem::size_of_val(&indices) as _,
            indices.as_ptr() as *const GLvoid,
            STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            8 * size_of::<f32>() as i32,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            8 * size_of::<f32>() as i32,
            (3 * size_of::<f32>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(1);

        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            8 * size_of::<f32>() as i32,
            (6 * size_of::<f32>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(2);
    };

    let mut texture: u32 = 0;
    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(TEXTURE_2D, texture);
    }

    unsafe {
        gl::TexParameteri(TEXTURE_2D, TEXTURE_WRAP_S, MIRRORED_REPEAT as GLint);
        gl::TexParameteri(TEXTURE_2D, TEXTURE_WRAP_T, MIRRORED_REPEAT as GLint);
        gl::TexParameteri(TEXTURE_2D, TEXTURE_MIN_FILTER, NEAREST as GLint);
        gl::TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as GLint);
    }

    let mut width: i32 = 0;
    let mut height: i32 = 0;
    let mut nr_channels: i32 = 0;

    let container_path = CString::new("assets/container.jpg").unwrap();
    let data = unsafe {
        stbi_load(
            container_path.as_ptr(),
            &mut width as *mut c_int,
            &mut height as *mut c_int,
            &mut nr_channels,
            0,
        )
    };

    if data != null_mut() {
        unsafe {
            gl::TexImage2D(
                TEXTURE_2D,
                0,
                gl::RGB.try_into().unwrap(),
                width,
                height,
                0,
                gl::RGB,
                UNSIGNED_BYTE,
                data as *const c_void,
            );
            gl::GenerateMipmap(TEXTURE_2D);
        }
    } else {
        eprintln!("Failed to load texture");
    }

    unsafe {
        stbi_image_free(data as *mut c_void);
    }

    while !window.should_close() {
        process_input(&mut window);

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(COLOR_BUFFER_BIT);

            our_shader.use_shader();
            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, UNSIGNED_INT, null());
            gl::BindVertexArray(0);
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
