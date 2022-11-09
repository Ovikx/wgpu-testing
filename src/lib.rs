use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder}
};

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>
}

impl State {
    async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // Create the WGPU instance that will create the Adapter and Surface
        let instance = wgpu::Instance::new(wgpu::Backends::all());

        // "Surface" is analogous to a window
        let surface = unsafe {instance.create_surface(window)};

        // Adapter is what's used to interface with the GPU
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false
            }
        ).await.unwrap();

        // Create device and command queue with architecture in mind
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None
            },
        None
        ).await.unwrap();

        //
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT, // Specifies how textures will be written to the screen
            format: surface.get_supported_formats(&adapter)[0], // How to store the surfaces on the GPU
            width: size.width, // width of the window
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo, // How to sync the surface with the display 
            alpha_mode: wgpu::CompositeAlphaMode::Auto
        };
        surface.configure(&device, &config);

        Self {
            surface,
            device,
            queue,
            config,
            size
        }
    }

    fn resize(&mut self, event: &WindowEvent) -> bool {
        todo!()
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        todo!()
    }

    fn update(&mut self) {
        todo!()
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        todo!()
    }
}

pub async fn run() {
    // Logger
    env_logger::init();

    // Window setup
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // State setup
    let mut state = State::new(&window).await;

    // Event loop
    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { ref event, window_id}
        if window_id == window.id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    ..
                },
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        },
        _ => {}
    });
}
