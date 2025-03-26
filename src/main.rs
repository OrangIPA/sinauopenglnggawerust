use std::{
    cell::RefCell,
    ffi::{CString, c_int, c_void},
    mem::{self, size_of},
    ptr::null_mut,
    rc::Rc,
};

use gl::{
    ARRAY_BUFFER, COLOR_BUFFER_BIT, DEPTH_BUFFER_BIT, ELEMENT_ARRAY_BUFFER, LINEAR,
    MIRRORED_REPEAT, NEAREST, STATIC_DRAW, TEXTURE_2D, TEXTURE_MAG_FILTER, TEXTURE_MIN_FILTER,
    TEXTURE_WRAP_S, TEXTURE_WRAP_T, TEXTURE1, UNSIGNED_BYTE,
    types::{GLint, GLuint, GLvoid},
};
use glfw::{Action, Context, Key};
use nalgebra_glm as glm;
use shader::Shader;
use stb_image::stb_image::{stbi_image_free, stbi_load, stbi_set_flip_vertically_on_load};

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

    gl::load_with(|s| window.get_proc_address(s));

    window.make_current();
    window.set_framebuffer_size_polling(true);

    let our_shader =
        Shader::new("shaders/vertex_shader.glsl", "shaders/fragment_shader.glsl").unwrap();

    #[rustfmt::skip]
    let vertices: [f32; 36 * 5] = [
        // positions      // texture coords
       -0.5, -0.5, -0.5,  0.0, 0.0,
        0.5, -0.5, -0.5,  1.0, 0.0,
        0.5,  0.5, -0.5,  1.0, 1.0,
        0.5,  0.5, -0.5,  1.0, 1.0,
       -0.5,  0.5, -0.5,  0.0, 1.0,
       -0.5, -0.5, -0.5,  0.0, 0.0,
   
       -0.5, -0.5,  0.5,  0.0, 0.0,
        0.5, -0.5,  0.5,  1.0, 0.0,
        0.5,  0.5,  0.5,  1.0, 1.0,
        0.5,  0.5,  0.5,  1.0, 1.0,
       -0.5,  0.5,  0.5,  0.0, 1.0,
       -0.5, -0.5,  0.5,  0.0, 0.0,
   
       -0.5,  0.5,  0.5,  1.0, 0.0,
       -0.5,  0.5, -0.5,  1.0, 1.0,
       -0.5, -0.5, -0.5,  0.0, 1.0,
       -0.5, -0.5, -0.5,  0.0, 1.0,
       -0.5, -0.5,  0.5,  0.0, 0.0,
       -0.5,  0.5,  0.5,  1.0, 0.0,
   
        0.5,  0.5,  0.5,  1.0, 0.0,
        0.5,  0.5, -0.5,  1.0, 1.0,
        0.5, -0.5, -0.5,  0.0, 1.0,
        0.5, -0.5, -0.5,  0.0, 1.0,
        0.5, -0.5,  0.5,  0.0, 0.0,
        0.5,  0.5,  0.5,  1.0, 0.0,
   
       -0.5, -0.5, -0.5,  0.0, 1.0,
        0.5, -0.5, -0.5,  1.0, 1.0,
        0.5, -0.5,  0.5,  1.0, 0.0,
        0.5, -0.5,  0.5,  1.0, 0.0,
       -0.5, -0.5,  0.5,  0.0, 0.0,
       -0.5, -0.5, -0.5,  0.0, 1.0,
   
       -0.5,  0.5, -0.5,  0.0, 1.0,
        0.5,  0.5, -0.5,  1.0, 1.0,
        0.5,  0.5,  0.5,  1.0, 0.0,
        0.5,  0.5,  0.5,  1.0, 0.0,
       -0.5,  0.5,  0.5,  0.0, 0.0,
       -0.5,  0.5, -0.5,  0.0, 1.0
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
            5 * size_of::<f32>() as i32,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            5 * size_of::<f32>() as i32,
            (3 * size_of::<f32>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(1);
    };

    unsafe {
        gl::TexParameteri(TEXTURE_2D, TEXTURE_WRAP_S, MIRRORED_REPEAT as GLint);
        gl::TexParameteri(TEXTURE_2D, TEXTURE_WRAP_T, MIRRORED_REPEAT as GLint);
        gl::TexParameteri(TEXTURE_2D, TEXTURE_MIN_FILTER, NEAREST as GLint);
        gl::TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as GLint);
    };

    let (mut width_container, mut height_container, mut nr_channels_container): (i32, i32, i32) =
        (0, 0, 0);
    let (mut width_face, mut height_face, mut nr_channels_face): (i32, i32, i32) = (0, 0, 0);

    let container_path = CString::new("assets/container.jpg").unwrap();
    let data_container = unsafe {
        stbi_load(
            container_path.as_ptr(),
            &mut width_container as *mut c_int,
            &mut height_container as *mut c_int,
            &mut nr_channels_container,
            0,
        )
    };

    let face_path = CString::new("assets/awesomeface.png").unwrap();
    let data_face = unsafe {
        stbi_set_flip_vertically_on_load(1);
        stbi_load(
            face_path.as_ptr(),
            &mut width_face as *mut c_int,
            &mut height_face as *mut c_int,
            &mut nr_channels_face,
            0,
        )
    };

    let mut texture_container: u32 = 0;
    let mut texture_face: u32 = 0;
    unsafe {
        gl::GenTextures(1, &mut texture_container);
        gl::BindTexture(TEXTURE_2D, texture_container);

        if data_container != null_mut() {
            gl::TexImage2D(
                TEXTURE_2D,
                0,
                gl::RGB.try_into().unwrap(),
                width_container,
                height_container,
                0,
                gl::RGB,
                UNSIGNED_BYTE,
                data_container as *const c_void,
            );
            gl::GenerateMipmap(TEXTURE_2D);
        } else {
            eprintln!("Failed to load container texture");
        }

        gl::ActiveTexture(TEXTURE1);
        gl::GenTextures(1, &mut texture_face);
        gl::BindTexture(TEXTURE_2D, texture_face);

        if data_face != null_mut() {
            gl::TexImage2D(
                TEXTURE_2D,
                0,
                gl::RGB.try_into().unwrap(),
                width_face,
                height_face,
                0,
                gl::RGBA,
                UNSIGNED_BYTE,
                data_face as *const c_void,
            );
            gl::GenerateMipmap(TEXTURE_2D);
        } else {
            eprintln!("Failed to load face texture");
        }
    };

    unsafe {
        stbi_image_free(data_face as *mut c_void);
        stbi_image_free(data_container as *mut c_void);
    }

    our_shader.use_shader();
    our_shader.set_int("texture1", 0);
    our_shader.set_int("texture2", 1);

    unsafe { gl::Enable(gl::DEPTH_TEST) };

    let cube_positions = vec![
        glm::vec3(0.0, 0.0, 0.0),
        glm::vec3(2.0, 5.0, -15.0),
        glm::vec3(-1.5, -2.2, -2.5),
        glm::vec3(-3.8, -2.0, -12.3),
        glm::vec3(2.4, -0.4, -3.5),
        glm::vec3(-1.7, 3.0, -7.5),
        glm::vec3(1.3, -2.0, -2.5),
        glm::vec3(1.5, 2.0, -2.5),
        glm::vec3(1.5, 0.2, -1.5),
        glm::vec3(-1.3, 1.0, -1.5),
    ];

    let camera = Rc::new(RefCell::new(CameraState {
        pos: (glm::vec3(0.0, 0.0, 3.0)),
        front: (glm::vec3(0.0, 0.0, -1.0)),
        up: (glm::vec3(0.0, 1.0, 0.0)),

        yaw: (-90.0),
        pitch: (0.0),
    }));
    let mut last_time = 0 as f32;

    window.set_cursor_mode(glfw::CursorMode::Disabled);

    let mut last_x = 400f32;
    let mut last_y = 300f32;
    let mut first_mouse = true;
    let camera_clone = camera.clone();
    window.set_cursor_pos_callback(move |_, x, y| {
        let mut cam = camera_clone.borrow_mut();
        if first_mouse {
            last_x = x as f32;
            last_y = y as f32;

            first_mouse = false;
        }

        let mut x_offset = x as f32 - last_x;
        let mut y_offset = last_y - y as f32;
        last_x = x as f32;
        last_y = y as f32;

        const SENSITIVITY: f32 = 0.1;
        x_offset *= SENSITIVITY;
        y_offset *= SENSITIVITY;

        cam.yaw += x_offset;
        cam.pitch += y_offset;

        cam.pitch = cam.pitch.clamp(-89.0, 90.0);

        let mut direction = glm::Vec3::zeros();
        direction.x = f32::cos(cam.yaw.to_radians()) * f32::cos(cam.pitch.to_radians());
        direction.y = f32::sin(cam.pitch.to_radians());
        direction.z = f32::sin(cam.yaw.to_radians()) * f32::cos(cam.pitch.to_radians());

        cam.front = glm::normalize(&direction);
    });

    while !window.should_close() {
        let current_time = glfw.get_time() as f32;
        let delta = current_time - last_time;
        last_time = current_time;
        process_input(&mut window, &mut camera.borrow_mut(), delta);

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);

            let view = glm::look_at(
                &camera.borrow().pos,
                &(camera.borrow().pos + camera.borrow().front),
                &camera.borrow().up,
            );
            our_shader.set_mat4("view", &view);

            let projection = glm::perspective(800. / 600., f32::to_radians(45.), 0.1, 100.);
            our_shader.set_mat4("projection", &projection);

            our_shader.use_shader();
            gl::BindVertexArray(vao);
            for (i, cube) in cube_positions.iter().enumerate() {
                let angle = 20. * i as f32;
                let mut model = glm::identity::<f32, 4>();
                model = glm::translate(&model, cube);
                model = glm::rotate(&model, f32::to_radians(angle), &glm::vec3(1.0, 0.3, 0.5));
                our_shader.set_mat4("model", &model);

                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
            gl::BindVertexArray(0);
        };

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_input(window: &mut glfw::Window, cam: &mut CameraState, delta: f32) {
    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true);
    }

    let camera_speed = 5. * delta;
    if window.get_key(Key::W) == Action::Press {
        cam.pos += camera_speed * cam.front;
    }
    if window.get_key(Key::S) == Action::Press {
        cam.pos -= camera_speed * cam.front;
    }
    if window.get_key(Key::A) == Action::Press {
        cam.pos -= glm::normalize(&(glm::cross(&cam.front, &cam.up))) * camera_speed;
    }
    if window.get_key(Key::D) == Action::Press {
        cam.pos += glm::normalize(&(glm::cross(&cam.front, &cam.up))) * camera_speed;
    }
}

struct CameraState {
    pub pos: glm::Vec3,
    pub front: glm::Vec3,
    pub up: glm::Vec3,

    pub yaw: f32,
    pub pitch: f32,
}
