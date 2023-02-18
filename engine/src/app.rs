use crate::*;

use std::time::{Duration, Instant};
use glium::glutin;
use glium::Surface;

pub struct EventLoopWrapper
{
	event_loop: Option<glutin::event_loop::EventLoop<()>>,
	process_input_fn: Box<dyn FnMut(*mut App, &glutin::event::Event<()>) -> Option<glutin::event_loop::ControlFlow>>,
	on_create_fn: Box<dyn FnMut(*mut App, &glium::Display)>,
	on_update_fn: Box<dyn FnMut(*mut App, &glium::Display, &mut glium::Frame)>,
	pub(crate) display: glium::Display,
}

impl EventLoopWrapper
{
	pub fn build<PI, OC, OU>(process_input: PI, on_create: OC, on_update: OU) -> Self
		where
		PI: 'static + FnMut(*mut App, &glutin::event::Event<()>) -> Option<glutin::event_loop::ControlFlow>,
		OC: 'static + FnMut(*mut App, &glium::Display),
		OU: 'static + FnMut(*mut App, &glium::Display, &mut glium::Frame),
	{
		let event_loop = glutin::event_loop::EventLoop::new();
		let wb = glutin::window::WindowBuilder::new()
			.with_inner_size(glutin::dpi::LogicalSize::new(640, 640));
		let cb = glutin::ContextBuilder::new()
			.with_depth_buffer(24);
		let display = glium::Display::new(wb, cb, &event_loop).unwrap();

		Self {
			event_loop: Some(event_loop),
			process_input_fn: Box::new(process_input),
			on_create_fn: Box::new(on_create),
			on_update_fn: Box::new(on_update),
			display,
		}
	}

	// this function "consumes" the EventLoopWrapper member of App. If, after the consumption,
	// another rendering session begins, the EventLoopWrapper needs to be rebuilt.
	pub fn run(mut self, app: *mut App)
	{
		(*self.on_create_fn)(app, &self.display);
		self.event_loop.unwrap().run(move |ev, _, control_flow| {
            // TODO control framerate
			let next_frame_time = Instant::now() + Duration::from_nanos(16_666_667);
			*control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

			// process_input processes events and if necessary returns a control flow variant, which is captured
			match (*self.process_input_fn)(app, &ev) {
					Some(x) => {
						*control_flow = x;
						return;
					},
					_ => ()
			}

			let mut target = self.display.draw();
			target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

			(*self.on_update_fn)(app, &self.display, &mut target);
			target.finish().unwrap();
		});
	}
}

pub struct App {
    pub(crate) elw: Option<EventLoopWrapper>,
    pub scene: scene::Scene,
}

impl App
{
	pub fn new<PI, OC, OU>(process_events: PI, on_create: OC, on_update: OU) -> Self
		where
		PI: 'static + FnMut(*mut App, &glutin::event::Event<()>) -> Option<glutin::event_loop::ControlFlow>,
		OC: 'static + FnMut(*mut App, &glium::Display),
		OU: 'static + FnMut(*mut App, &glium::Display, &mut glium::Frame),
	{
		let elw = EventLoopWrapper::build(process_events, on_create, on_update);
		Self { elw: Some(elw), scene: scene::Scene::new() }
	}

	pub fn run(&mut self) {
		self.elw.take().unwrap().run(self as *mut App);
	}
}
