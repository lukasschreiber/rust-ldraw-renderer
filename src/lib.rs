mod events;
mod rendering;
mod utils;

use events::{Rendering, RenderingUserEvent};
use three_d::{Window, WindowError, WindowSettings};
use wasm_bindgen::prelude::*;
use winit::event_loop::{EventLoopProxy, EventLoopWindowTarget};

use crate::rendering::render_instanced_cubes;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[non_exhaustive]
#[wasm_bindgen]
pub struct RenderingNever(Rendering<()>);

#[wasm_bindgen]
impl RenderingNever {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(Rendering::new())
    }

    #[wasm_bindgen]
    pub fn get_proxy(&self) -> CustomEventLoopProxy {
        CustomEventLoopProxy(self.0.get_proxy(), 0)
    }

    #[wasm_bindgen]
    pub fn run(self) {
        self.0.run()
    }
}

#[wasm_bindgen]
pub struct CustomEventLoopProxy(EventLoopProxy<RenderingUserEvent<()>>, usize);

#[wasm_bindgen]
impl CustomEventLoopProxy {
    #[wasm_bindgen]
    pub fn send_event(&self) {
        self.0
            .send_event(RenderingUserEvent::Other(()))
            .unwrap_or_else(|_| panic!("Something went horribly wrong!"));
    }

    #[wasm_bindgen]
    pub fn create_window(&mut self, canvas_id: &str) -> usize {
        let value = create_window(canvas_id);
        let id = self.1;
        self.0
            .send_event(RenderingUserEvent::InternalCreateWindow(id, value))
            .unwrap_or_else(|_| panic!("Something went horribly wrong!"));
        self.1 += 1;
        id
    }

    #[wasm_bindgen]
    pub fn delete_window(&self, id: usize) {
        self.0
            .send_event(RenderingUserEvent::InternalDeleteWindow(id))
            .unwrap_or_else(|_| panic!("Something went horribly wrong!"));
    }

    #[wasm_bindgen]
    pub fn update_prop(&self, value: u8) {
        self.0
            .send_event(RenderingUserEvent::InternalUpdateProps(value))
            .unwrap_or_else(|_| panic!("Something went horribly wrong!"));
    }
}

#[wasm_bindgen]
pub struct RenderingParams {
    red: i32,
}

#[wasm_bindgen]
impl RenderingParams {
    #[wasm_bindgen(constructor)]
    pub fn new(val: i32) -> RenderingParams {
        RenderingParams { red: val }
    }

    pub fn get(&self) -> i32 {
        self.red
    }

    pub fn set(&mut self, val: i32) {
        self.red = val;
    }
}

pub fn create_window(
    canvas_id: &str,
) -> Box<
    dyn FnOnce(
        &EventLoopWindowTarget<RenderingUserEvent<()>>,
    ) -> Box<
        dyn FnMut(
            &winit::event::Event<RenderingUserEvent<()>>,
            &winit::event_loop::EventLoopWindowTarget<RenderingUserEvent<()>>,
            &mut winit::event_loop::ControlFlow,
        ),
    >,
> {
    wasm_logger::init(wasm_logger::Config::default());

    let websys_window = web_sys::window()
        .ok_or(WindowError::WindowCreation)
        .unwrap();
    let document = websys_window
        .document()
        .ok_or(WindowError::DocumentMissing)
        .unwrap();
    let canvas_element = document
        .get_element_by_id(canvas_id)
        .expect("settings doesn't contain canvas and DOM doesn't have a canvas element either")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|e| WindowError::CanvasConvertFailed(format!("{:?}", e)))
        .unwrap();

    println!("canvas {}", canvas_element.id());

    // callback should have properties as struct
    let callback = Box::new(
        |event_loop: &EventLoopWindowTarget<RenderingUserEvent<()>>| {
            let window = Window::from_event_loop(
                WindowSettings {
                    title: "Instanced Shapes!".to_string(),
                    max_size: Some((1280, 720)),
                    canvas: Some(canvas_element),
                    ..Default::default()
                },
                event_loop,
            )
            .unwrap();

            render_instanced_cubes(window)
        },
    );
    callback
}
