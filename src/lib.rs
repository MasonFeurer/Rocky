#![allow(dead_code)]

pub mod cam;
pub mod input;
pub mod math;
pub mod model;
pub mod player;
pub mod texture;

use crate::input::FrameInput;
use crate::math::{Mat4, Vec3};
use crate::player::Player;
use texture::{Texture, DEPTH_TEXTURE_FORMAT};
use wgpu::util::DeviceExt;
use winit::{
    dpi::PhysicalSize,
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub struct Uniform<T> {
    pub buffer: wgpu::Buffer,
    pub data: T,
}
impl<T: bytemuck::NoUninit> Uniform<T> {
    pub fn new(device: &wgpu::Device, data: T) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("buffer"),
            contents: bytemuck::cast_slice(&[data]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        Self { buffer, data }
    }

    pub fn write(&self, queue: &mut wgpu::Queue) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.data]));
    }

    pub fn binding(&self, binding: u32) -> wgpu::BindGroupEntry {
        wgpu::BindGroupEntry {
            binding,
            resource: self.buffer.as_entire_binding(),
        }
    }
    pub fn bind_layout(
        &self,
        binding: u32,
        visibility: wgpu::ShaderStages,
    ) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding,
            visibility,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }
    }
}

struct Projection {
    fov: f32,
    aspect: f32,
    near: f32,
    far: f32,
}
impl Projection {
    pub fn default(aspect: f32) -> Self {
        Self {
            fov: 70.0,
            aspect,
            near: 0.01,
            far: 1000.0,
        }
    }

    pub fn matrix(&self) -> Mat4 {
        Mat4::projection(self.fov.to_radians(), self.aspect, self.near, self.far)
    }
}

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    depth_texture: Texture,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,

    bind_group0: wgpu::BindGroup,
    bind_group1: wgpu::BindGroup,

    texture: Texture,
    player: Player,
    projection: Projection,
    view_mat_uniform: Uniform<Mat4>,
    proj_mat_uniform: Uniform<Mat4>,
}
impl State {
    async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        // Handle to a presentable surface
        let surface = unsafe { instance.create_surface(window) };
        // Handle to the graphics device
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        // device: Open connection to graphics device
        // queue: Handle to a command queue on the device
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };
        surface.configure(&device, &config);

        // Create depth texture
        let depth_texture = Texture::create_depth_texture(&device, size.width, size.height);

        // Done with setup
        // Now create textures

        let image = image::load_from_memory(include_bytes!("../res/happy-tree.png")).unwrap();
        let texture = Texture::from_image(&device, &queue, &image, Some("texture"));

        // Create uniforms
        let player = Player::new(Vec3::new(0.0, 0.0, 4.0), Vec3::all(0.0));
        let projection = {
            let win_size = window.inner_size();
            let aspect = win_size.width as f32 / win_size.height as f32;
            Projection::default(aspect)
        };

        let view_mat_uniform = Uniform::new(&device, player.cam.matrix());
        let proj_mat_uniform = Uniform::new(&device, projection.matrix());

        // Create bind groups
        let bind_group0_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("bind_group0_layout"),
                entries: &[texture.bind_layout(0), texture.sampler_bind_layout(1)],
            });
        let bind_group0 = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("bind_group0"),
            layout: &bind_group0_layout,
            entries: &[texture.binding(0), texture.sampler_binding(1)],
        });

        let bind_group1_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("bind_group1_layout"),
                entries: &[
                    view_mat_uniform.bind_layout(0, wgpu::ShaderStages::VERTEX),
                    proj_mat_uniform.bind_layout(1, wgpu::ShaderStages::VERTEX),
                ],
            });
        let bind_group1 = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("bind_group1"),
            layout: &bind_group1_layout,
            entries: &[view_mat_uniform.binding(0), proj_mat_uniform.binding(1)],
        });

        // Create shaders
        let shader = device.create_shader_module(wgpu::include_wgsl!("../res/shader.wgsl"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("render_pipeline_layout"),
                bind_group_layouts: &[&bind_group0_layout, &bind_group1_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("render_pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[model::Vertex::buffer_layout()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                // cull_mode: Some(wgpu::Face::Back),
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: DEPTH_TEXTURE_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        // Create models
        let (vertices, indices) = model::default_model();
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("vertex buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("index buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            depth_texture,

            vertex_buffer,
            index_buffer,

            bind_group0,
            bind_group1,

            texture,
            player,
            projection,
            view_mat_uniform,
            proj_mat_uniform,
        }
    }

    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width == 0 || new_size.height == 0 {
            return;
        }
        self.size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);

        self.projection.aspect = new_size.width as f32 / new_size.height as f32;
        self.proj_mat_uniform.data = self.projection.matrix();
        self.proj_mat_uniform.write(&mut self.queue);

        self.depth_texture =
            Texture::create_depth_texture(&self.device, self.size.width, self.size.height);
    }

    fn update(&mut self, input: &FrameInput) {
        self.player.update(input);
        self.view_mat_uniform.data = self.player.cam.matrix();
        self.view_mat_uniform.write(&mut self.queue);
    }

    fn render(&mut self, _input: &FrameInput) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("render encoder"),
            });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_texture.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: None,
            }),
        });
        render_pass.set_pipeline(&self.render_pipeline);

        render_pass.set_bind_group(0, &self.bind_group0, &[]);
        render_pass.set_bind_group(1, &self.bind_group1, &[]);

        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw_indexed(0..model::NUM_INDICES as u32, 0, 0..1);
        std::mem::drop(render_pass);

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

pub async fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("WGPU Voxel Game")
        .build(&event_loop)
        .unwrap();
    window.set_cursor_visible(false);

    let mut state = State::new(&window).await;
    let mut frame_input = FrameInput::new();

    event_loop.run(move |event, _, control_flow| match event {
        e if frame_input.capture_event(&e) => {}

        Event::RedrawRequested(_) => {
            state.update(&frame_input);
            match state.render(&frame_input) {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => eprintln!("{e:?}"),
            };
            frame_input.clear();
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            WindowEvent::Resized(new_size) => state.resize(new_size),
            _ => {}
        },
        _ => {}
    });
}
