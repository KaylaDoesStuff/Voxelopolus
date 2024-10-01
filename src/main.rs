// Import the necessary crates
extern crate glfw;
use crate::graphics::objects::VAOData;
use crate::graphics::objects::IBOData;
use crate::graphics::objects::VBOData;
use crate::objects::objects::index_data_voxel;
use crate::objects::objects::cube_voxel;
use crate::objects::objects::VoxelData;
mod objects;
mod graphics;

use glfw::{Action, Context, Key};

const fall: f32 = -5.0;

fn shape_as_vbo(vd: VoxelData) -> VBOData {
    let vertex_data = vd.points;
    let vertex_size = 3;
    let vbo = VBOData::new(vertex_data, vertex_size);
    vbo
}

fn main() {
    // Initialize GLFW
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Create a window
    let (mut window, _) = glfw.create_window(800, 600, "My Window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    let voxel_vbo = shape_as_vbo(cube_voxel.clone());
    let voxel_ibo = IBOData::new(index_data_voxel.clone());
    let voxel_vao = VAOData::new();
    voxel_vao.bind();
    voxel_vbo.bind();

    // Main loop
    while !window.should_close() {
        // Swap front and back buffers
        window.swap_buffers();

        // Poll for and process events
        glfw.poll_events();

        // Check for key presses
        match window.get_key(Key::Escape) {
            Action::Press => window.set_should_close(true),
            _ => (),
        }
    }
}