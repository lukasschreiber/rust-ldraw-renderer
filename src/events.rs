use std::collections::HashMap;

use winit::{
    event::Event,
    event_loop::{EventLoop, EventLoopBuilder, EventLoopProxy, EventLoopWindowTarget},
};

use crate::utils;

pub enum RenderingUserEvent<Q: 'static> {
    InternalCreateWindow(
        usize,
        Box<
            dyn FnOnce(
                &EventLoopWindowTarget<RenderingUserEvent<Q>>,
            ) -> Box<
                dyn FnMut(
                    &winit::event::Event<RenderingUserEvent<Q>>,
                    &winit::event_loop::EventLoopWindowTarget<RenderingUserEvent<Q>>,
                    &mut winit::event_loop::ControlFlow,
                ),
            >,
        >,
    ),
    InternalDeleteWindow(usize),
    InternalUpdateProps(u8),
    Other(Q),
}

impl<Q: Clone + 'static> Clone for RenderingUserEvent<Q> {
    fn clone(&self) -> Self {
        match self {
            Self::InternalCreateWindow(_, _) => panic!("can't clone InternalCreateWindow"),
            Self::InternalDeleteWindow(_) => panic!("can't clone InternalDeleteWindow"),
            Self::InternalUpdateProps(arg0) => Self::InternalUpdateProps(arg0.clone()),
            Self::Other(arg0) => Self::Other(arg0.clone()),
        }
    }
}

pub struct Rendering<Q: 'static> {
    event_loop: EventLoop<RenderingUserEvent<Q>>,
}

impl<Q: 'static> Rendering<Q> {
    pub fn new() -> Self {
        utils::set_panic_hook();

        Self {
            event_loop: EventLoopBuilder::with_user_event().build(),
        }
    }

    pub fn get_proxy(&self) -> EventLoopProxy<RenderingUserEvent<Q>> {
        self.event_loop.create_proxy()
    }

    pub fn run(self) -> ! {
        let mut handlers: HashMap<
            usize,
            Box<
                dyn FnMut(
                    &winit::event::Event<RenderingUserEvent<Q>>,
                    &winit::event_loop::EventLoopWindowTarget<RenderingUserEvent<Q>>,
                    &mut winit::event_loop::ControlFlow,
                ),
            >,
        > = HashMap::new();

        self.event_loop.run(move |event, target, control_flow| {
            match event {
                Event::UserEvent(RenderingUserEvent::InternalCreateWindow(id, callback)) => {
                    handlers.insert(id, callback(target));
                }
                Event::UserEvent(RenderingUserEvent::InternalDeleteWindow(id)) => {
                    handlers.remove(&id);
                }
                event => {
                    // TODO FIXME remove our custom type wrapper RenderingUserEvent and then maybe we could use an FnOnce above
                    for handler in handlers.values_mut() {
                        handler(&event, target, control_flow);
                    }
                }
            }
        })
    }
}
