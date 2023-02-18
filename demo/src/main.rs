use engine::*;

use glium::glutin;
use glium::Surface;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(640, 480));
    let cp = glutin::ContextBuilder::new(); //.with_depth_buffer(24);
    let display = glium::Display::new(wb, cp, &event_loop).unwrap();


    let vertex_shader_src = r#"
        #version 140

        in vec3 position;
        in vec3 color;
        in vec3 normal;
        in vec2 tex_coord;

        out vec3 vert_color;

        void main() {
            gl_Position = vec4(position, 1.0);
            vert_color = color;
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec3 vert_color;
        out vec4 color;

        void main() {
            color = vec4(vert_color, 1.0);
        }
    "#;

    let renderable = renderer::Renderable::cube(vertex_shader_src, fragment_shader_src, &display);

    let mut scene = scene::Scene::new();
    scene.ecs.add_entity(1);
    scene.ecs.add_component(1, renderable);


    event_loop.run(move |ev, _, control_flow| {

        // rendering
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        let shape: &renderer::Renderable = scene.ecs.get_component::<renderer::Renderable>(1).unwrap();
        renderer::render(shape, &mut target, &display);

        target.finish().unwrap();

        let next_frame_time = std::time::Instant::now()
            + std::time::Duration::from_nanos(16_666_667);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            _ => (),
        }

    });
}
