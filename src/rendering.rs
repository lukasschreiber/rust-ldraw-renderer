use three_d::{
    degrees, vec3, AmbientLight, Camera, ClearState, Color, CpuMaterial, CpuMesh, Cull,
    DirectionalLight, Event, FrameOutput, Gm, Mesh, OrbitControl, PhysicalMaterial, Vec3, Viewport,
    Window,
};

use crate::{events::RenderingUserEvent, parser::part::LDrawBrick};

// should take some kind of properties as struct
pub fn render_brick(
    window: Window,
    brick: LDrawBrick,
) -> Box<
    dyn FnMut(
        &winit::event::Event<RenderingUserEvent<()>>,
        &winit::event_loop::EventLoopWindowTarget<RenderingUserEvent<()>>,
        &mut winit::event_loop::ControlFlow,
    ),
> {
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(60.00, 50.0, 60.0), // camera position
        vec3(0.0, 0.0, 0.0),     // camera target
        vec3(0.0, 1.0, 0.0),     // camera up
        degrees(45.0),
        0.1,
        1000.0,
    );
    let mut control = OrbitControl::new(vec3(0.0, 0.0, 0.0), 1.0, 1000.0);

    let light0 = DirectionalLight::new(&context, 0.5, Color::WHITE, &vec3(0.0, -0.5, -0.5));
    let light1 = DirectionalLight::new(&context, 0.5, Color::WHITE, &vec3(0.0, 0.5, 0.5));
    let amb_light = AmbientLight::new(&context, 0.5, Color::WHITE);

    let mut positions = Vec::new();
    positions.push(Vec3::new(0.0, 0.0, 0.0));
    positions.push(Vec3::new(0.0, 1.0, 0.0));
    positions.push(Vec3::new(2.0, 0.0, 0.0));
    positions.push(Vec3::new(2.0, 0.0, 2.0));

    let mut brick_tri_mesh = CpuMesh {
        positions: three_d::Positions::F32(positions),
        indices: three_d::Indices::U8(Vec::from([0, 1, 2, 3, 2, 1])),
        normals: None,
        tangents: None,
        uvs: None,
        colors: None,
    };

    brick_tri_mesh.compute_normals();

    let mut brick_mesh = Gm::new(
        Mesh::new(&context, &brick_tri_mesh),
        PhysicalMaterial::new(
            &context,
            &CpuMaterial {
                albedo: Color {
                    r: 0,
                    g: 0,
                    b: 255,
                    a: 255,
                },
                ..Default::default()
            },
        ),
    );

    brick_mesh.material.render_states.cull = Cull::Back;

    let mut red: u8 = 0;

    let inner_callback: Box<
        dyn FnMut(
            &winit::event::Event<RenderingUserEvent<()>>,
            &winit::event_loop::EventLoopWindowTarget<RenderingUserEvent<()>>,
            &mut winit::event_loop::ControlFlow,
        ),
    > = Box::new(
        window.get_render_loop::<RenderingUserEvent<()>, _>(move |mut frame_input| {
            for event in frame_input.events.iter_mut() {
                match event {
                    Event::UserEvent(RenderingUserEvent::InternalUpdateProps(value)) => {
                        red = *value
                    }
                    _ => {}
                }
            }

            let viewport = Viewport {
                x: 0,
                y: 0,
                width: frame_input.viewport.width,
                height: frame_input.viewport.height,
            };
            camera.set_viewport(viewport);

            // Camera control must be after the gui update.
            control.handle_events(&mut camera, &mut frame_input.events);

            brick_mesh.material.albedo = Color {
                r: red,
                g: 128,
                b: 128,
                a: 255,
            };

            // Then, based on whether or not we render the instanced brick_meshs, collect the renderable
            // objects.

            frame_input
                .screen()
                .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
                .render(
                    &camera,
                    brick_mesh.into_iter(),
                    &[&light0, &light1, &amb_light],
                );

            FrameOutput::default()
        }),
    );
    inner_callback
}
