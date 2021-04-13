use winit::{
    event::{ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

mod shader;
pub use shader::*;

unsafe fn as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>())
}

async fn run(
    event_loop: EventLoop<()>,
    window: Window,
    swapchain_format: wgpu::TextureFormat,
    rx: std::sync::mpsc::Receiver<notify::DebouncedEvent>,
) {
    let size = window.inner_size();
    let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

    let mut surface = Some(unsafe { instance.create_surface(&window) });

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            // Request an adapter which can render to our surface
            compatible_surface: surface.as_ref(),
        })
        .await
        .expect("Failed to find an appropriate adapter");

    // Create the logical device and command queue
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::PUSH_CONSTANTS,
                limits: wgpu::Limits {
                    max_push_constant_size: 4096,
                    ..Default::default()
                },
            },
            None,
        )
        .await
        .expect("Failed to create device");

    let mut sc_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Mailbox,
    };

    let mut swap_chain = surface
        .as_ref()
        .map(|surface| device.create_swap_chain(&surface, &sc_desc));

    let mut shader = Shader::new(&device, swapchain_format);

    let start = std::time::Instant::now();
    let (mut cursor_x, mut cursor_y) = (0.0, 0.0);
    let (mut drag_start_x, mut drag_start_y) = (0.0, 0.0);
    let (mut drag_end_x, mut drag_end_y) = (0.0, 0.0);
    let mut mouse_left_pressed = false;
    let mut mouse_left_clicked = false;

    let mut frame_count = 0.0;

    event_loop.run(move |event, _, control_flow| {
        // Have the closure take ownership of the resources.
        // `event_loop.run` never returns, therefore we must do this to ensure
        // the resources are properly cleaned up.
        let _ = (&instance, &adapter, &shader);

        *control_flow = ControlFlow::Poll;
        match event {
            Event::MainEventsCleared => {
                for n in rx.try_recv() {
                    match n {
                        notify::DebouncedEvent::Write(_) => {
                            shader.rebuild(&device, swapchain_format);
                        }
                        _ => {}
                    }

                }
                window.request_redraw();
            }
            Event::Resumed => {
                let s = unsafe { instance.create_surface(&window) };
                swap_chain = Some(device.create_swap_chain(&s, &sc_desc));
                surface = Some(s);
            }
            Event::Suspended => {
                surface = None;
                swap_chain = None;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                // Recreate the swap chain with the new size
                sc_desc.width = size.width;
                sc_desc.height = size.height;
                if let Some(surface) = &surface {
                    swap_chain = Some(device.create_swap_chain(surface, &sc_desc));
                }
            }
            Event::RedrawRequested(_) => {
                if let Some(swap_chain) = &mut swap_chain {
                    let frame = swap_chain
                        .get_current_frame()
                        .expect("Failed to acquire next swap chain texture")
                        .output;
                    let mut encoder = device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                    {
                        let clear_color = wgpu::Color {
                            r: 0.2,
                            g: 0.2,
                            b: 0.25,
                            a: 1.0,
                        };
                        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: None,
                            color_attachments: &[wgpu::RenderPassColorAttachment {
                                view: &frame.view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(clear_color),
                                    store: true,
                                },
                            }],
                            depth_stencil_attachment: None,
                        });
                        let push_constants = ShaderConstants {
                            width: window.inner_size().width as _,
                            height: window.inner_size().height as _,
                            frame: frame_count,
                            time: start.elapsed().as_secs_f32(),
                            cursor_x,
                            cursor_y,
                            drag_start_x,
                            drag_start_y,
                            drag_end_x,
                            drag_end_y,
                            mouse_left_pressed,
                            mouse_left_clicked,
                        };
                        mouse_left_clicked = false;
                        rpass.set_pipeline(shader.pipeline());
                        rpass.set_push_constants(wgpu::ShaderStage::all(), 0, unsafe {
                            as_u8_slice(&push_constants)
                        });
                        rpass.draw(0..3, 0..1);

                        frame_count += 1.0;
                    }

                    queue.submit(Some(encoder.finish()));
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event:
                    WindowEvent::MouseInput {
                        state,
                        button: MouseButton::Left,
                        ..
                    },
                ..
            } => {
                mouse_left_pressed = state == ElementState::Pressed;
                if mouse_left_pressed {
                    drag_start_x = cursor_x;
                    drag_start_y = cursor_y;
                    drag_end_x = cursor_x;
                    drag_end_y = cursor_y;
                    mouse_left_clicked = true;
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                cursor_x = position.x as f32;
                cursor_y = position.y as f32;
                if mouse_left_pressed {
                    drag_end_x = cursor_x;
                    drag_end_y = cursor_y;
                }
            }
            _ => {}
        }
    });
}

fn main() {
    let event_loop = EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_title("Rust GPU - wgpu")
        .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0))
        .build(&event_loop)
        .unwrap();
    // join filesd
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let shader_path = root.join("src/shader.wgsl");

    use notify::{Watcher};
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = notify::watcher(tx, std::time::Duration::from_millis(500)).unwrap();
    watcher
        .watch(&shader_path, notify::RecursiveMode::NonRecursive)
        .unwrap();

    let format = wgpu::TextureFormat::Bgra8UnormSrgb;

    pollster::block_on(run(event_loop, window, format, rx));
}
