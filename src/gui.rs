use imgui::im_str;
use imgui::Context as ImContext;
use imgui_glfw_rs::glfw;
use imgui_glfw_rs::glfw::Context;
use imgui_glfw_rs::glfw::Glfw;
use imgui_glfw_rs::imgui;
use imgui_glfw_rs::ImguiGLFW;

use tracing::*;

use crate::boat_state::BOAT_STATE;

pub fn run() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(2, 1));

    let (mut window, events) = glfw
        .create_window(1024, 768, "Can Monitor", glfw::WindowMode::Windowed)
        .expect("Failed to create window");

    window.make_current();
    window.set_all_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
        gl::ClearColor(0.1, 0.1, 0.1, 1.0);
    }

    let mut imgui = ImContext::create();

    let mut imgui_glfw = ImguiGLFW::new(&mut imgui, &mut window);

    while !window.should_close() {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let ui = imgui_glfw.frame(&mut window, &mut imgui);

        let data = BOAT_STATE.lock().unwrap().clone();

        let size = window.get_size();

        ui.window(im_str!("Can Debugger"))
            .size([size.0 as f32, size.1 as f32], imgui::Condition::Always)
            .position([0.0, 0.0], imgui::Condition::Appearing)
            .build(|| {
                ui.text(format!("{data:#?}"));
            });

        ui.show_metrics_window(&mut true);

        imgui_glfw.draw(ui, &mut window);

        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            imgui_glfw.handle_event(&mut imgui, &event);
        }
    }
}
