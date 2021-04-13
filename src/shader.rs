#[derive(Copy, Clone)]
#[allow(unused_attributes)]
// #[spirv(block)]
pub struct ShaderConstants {
    pub width: f32,
    pub height: f32,
    pub frame: f32,
    pub time: f32,
    pub cursor_x: f32,
    pub cursor_y: f32,
    pub drag_start_x: f32,
    pub drag_start_y: f32,
    pub drag_end_x: f32,
    pub drag_end_y: f32,
    pub mouse_left_pressed: bool,
    pub mouse_left_clicked: bool,
}

pub struct Shader {
    pipeline_layout: wgpu::PipelineLayout,
    pipeline: wgpu::RenderPipeline,
}

fn create_pipeline(
    device: &wgpu::Device,
    pipeline_layout: &wgpu::PipelineLayout,
    shader: &wgpu::ShaderModule,
    format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[format.into()],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
    });

    render_pipeline
}

fn create_shader(device: &wgpu::Device) -> wgpu::ShaderModule {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let shader_path = root.join("src/shader.wgsl");

    let prelude_path = root.join("src/prelude.wgsl");

    let prelude = std::fs::read_to_string(prelude_path).unwrap();
    let shader = std::fs::read_to_string(&shader_path).unwrap();

    let source = format!("{}{}", prelude, shader);

    let shader_desc = wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(&source)),
        flags: wgpu::ShaderFlags::all(),
    };

    device.create_shader_module(&shader_desc)
}

impl Shader {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[wgpu::PushConstantRange {
                stages: wgpu::ShaderStage::all(),
                range: 0..std::mem::size_of::<ShaderConstants>() as u32,
            }],
        });
        let shader = create_shader(device);

        Self {
            pipeline: create_pipeline(device, &pipeline_layout, &shader, format),
            pipeline_layout,
        }
    }

    pub fn rebuild(&mut self, device: &wgpu::Device, format: wgpu::TextureFormat) {
        let shader = create_shader(device);
        self.pipeline = create_pipeline(device, &self.pipeline_layout, &shader, format);
    }

    pub fn pipeline(&self) -> &wgpu::RenderPipeline {
        &self.pipeline
    }
}
