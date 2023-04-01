use three_d::{
    degrees, vec3, Camera, ClearState, Color, CpuMaterial, CpuMesh, DirectionalLight, Event,
    FrameOutput, Geometry, Gm, InstancedMesh, Instances, Mat4, Object, OrbitControl,
    PhysicalMaterial, Rad, Viewport, Window,
};

use crate::events::RenderingUserEvent;

// should take some kind of properties as struct
pub fn render_instanced_cubes(
    window: Window,
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

    let light0 = DirectionalLight::new(&context, 1.0, Color::WHITE, &vec3(0.0, -0.5, -0.5));
    let light1 = DirectionalLight::new(&context, 1.0, Color::WHITE, &vec3(0.0, 0.5, 0.5));

    // Instanced mesh object, initialise with empty instances.
    let mut instanced_mesh = Gm::new(
        InstancedMesh::new(&context, &Instances::default(), &CpuMesh::cube()),
        PhysicalMaterial::new(
            &context,
            &CpuMaterial {
                albedo: Color {
                    r: 0,
                    g: 128,
                    b: 128,
                    a: 255,
                },
                ..Default::default()
            },
        ),
    );
    instanced_mesh.set_animation(|time| Mat4::from_angle_x(Rad(time)));

    let side_count = 4;
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

            instanced_mesh.material.albedo = Color {
                r: red,
                g: 128,
                b: 128,
                a: 255,
            };

            // Ensure we have the correct number of cubes, does no work if already correctly sized.
            // only rerender if any props have changed
            let count = side_count * side_count * side_count;

            if instanced_mesh.instance_count() != count as u32 {
                instanced_mesh.set_instances(&Instances {
                    transformations: (0..count)
                        .map(|i| {
                            let x = (i % side_count) as f32;
                            let y = ((i as f32 / side_count as f32).floor() as usize % side_count)
                                as f32;
                            let z = (i as f32 / side_count.pow(2) as f32).floor();
                            Mat4::from_translation(
                                3.0 * vec3(x, y, z)
                                    - 1.5 * (side_count as f32) * vec3(1.0, 1.0, 1.0),
                            )
                        })
                        .collect(),
                    ..Default::default()
                });
            }

            // Always update the transforms for both the normal cubes as well as the instanced versions.
            // This shows that the difference in frame rate is not because of updating the transforms
            // and shows that the performance difference is not related to how we update the cubes.
            let time = (frame_input.accumulated_time * 0.001) as f32;
            instanced_mesh.animate(time);

            // Then, based on whether or not we render the instanced cubes, collect the renderable
            // objects.
            let render_objects: Vec<&dyn Object> = instanced_mesh.into_iter().collect();

            frame_input
                .screen()
                .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
                .render(&camera, render_objects, &[&light0, &light1]);

            FrameOutput::default()
        }),
    );
    inner_callback
}
