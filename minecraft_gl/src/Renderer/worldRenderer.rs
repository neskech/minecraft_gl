use std::collections::HashSet;
use image::GenericImageView;
use wgpu::util::DeviceExt;

use crate::Scene::camera::Camera;
use crate::World::chunk::{CHUNK_BOUNDS_X, CHUNK_BOUNDS_Y, CHUNK_BOUNDS_Z, TOTAL_CHUNK_SIZE};
use crate::Util::atlas::TextureAtlas;
use crate::Util::resource::ResourceManager;
use crate::World::chunk::Chunk;

pub const BLOCK_TEXTURE_RESOLUTION: u32 = 64;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex{
   pub Data: u32
}

#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniform {
    // We can't use cgmath with bytemuck directly so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    view_proj: [[f32; 4]; 4],
    chunk_pos: [f32; 2],
    sprite_dimensions: f32,
    tex_size: [f32; 2],
    atlas_cols: f32
}

impl Vertex {
    pub fn Attributes<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress, // 1.
            step_mode: wgpu::VertexStepMode::Vertex, // Indexed by vertex # or instance # ?
            attributes: &[ 
                wgpu::VertexAttribute {
                    offset: 0, 
                    shader_location: 0, 
                    format: wgpu::VertexFormat::Uint32,
                },
            ]
        }
    }
}

pub struct WorldRenderer{
    RenderPipeline: wgpu::RenderPipeline,
    VertexBuffer: wgpu::Buffer,
    IndexBuffer: wgpu::Buffer,
    UniformBuffer: wgpu::Buffer,
    TextureBindGroup: wgpu::BindGroup,
    UniformBindGroup: wgpu::BindGroup,
    TextureAtlas: TextureAtlas,
}

//TODO change all the errors to be Result<_, Str&> to avoid heap allcoation
impl WorldRenderer{
    pub fn New(resourceManager: &mut ResourceManager, atlas: TextureAtlas, device: &wgpu::Device, queue: &wgpu::Queue, config: &wgpu::SurfaceConfiguration) -> Self {

        let path = "./minecraft_gl/assets/shaders/world.glsl";
        let shader = resourceManager.GetShader(path, device);

        let texture = &atlas.Texture;
        //create a view so we can use the texture
        let diffuse_texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        //Set the texture parameters. All textures are 3D by default, can't be changed
        let diffuse_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT, //Visible to the fragment
                        ty: wgpu::BindingType::Texture {
                            multisampled: false, 
                            view_dimension: wgpu::TextureViewDimension::D2, //2D view dimensions
                            sample_type: wgpu::TextureSampleType::Float { filterable: true }, //Sample floating point numbers from the texture
                        },
                        count: None, //For arrays
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT, //Visible to the fragment
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: None,
            });

        let diffuse_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&diffuse_texture_view), //the view
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_sampler), //and the sampler
                    }
                ],
                label: None,
            }
        );

        let uniform_buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("uniform Buffer"),
                size: 1,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: true,
            }
        );

        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("uniform_bind_group_layout"),
        });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                }
            ],
            label: Some("uniform_bind_group"),
        });    

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("World Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout, &uniform_bind_group_layout],
                push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("World Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader.as_deref().unwrap(),
                entry_point: "vs_main",  //vs shader entry point
                buffers: &[Vertex::Attributes()], 
            },
            fragment: Some(wgpu::FragmentState { 
                module: shader.as_deref().unwrap(),
                entry_point: "fs_main", //fs shader entry point
                targets: &[Some(wgpu::ColorTargetState {  //What surfaces should we draw to // target?
                    format: config.format, //Use same format as surface for easy copying to its texture
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL, //Write R, G, B, and A
             })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, //every 3 vertices = one triangle
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, 
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: 1, // number of samples
                mask: !0, // which samples should be active. !0 = all of them
                alpha_to_coverage_enabled: false, // related to antialising
            },
            multiview: None, // related to arrray textures
        });

        let vertex_buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("Camera Buffer"),
                size: (TOTAL_CHUNK_SIZE * 6 * 4) as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: true,
        });

        let maxNumQuads: usize = (CHUNK_BOUNDS_X * CHUNK_BOUNDS_Y * CHUNK_BOUNDS_Z * 6) as usize;
        let mut indices: Vec<u16> = vec![0; maxNumQuads * 6];
        for i in 0..maxNumQuads {
            let c = i as u16;
            indices[0 + i * 6] = 0 + c * 4;
            indices[1 + i * 6] = 1 + c * 4;
            indices[2 + i * 6] = 3 + c * 4;
            indices[3 + i * 6] = 0 + c * 4;
            indices[4 + i * 6] = 3 + c * 4;
            indices[5 + i * 6] = 2 + c * 4;
        }

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&indices.as_slice()),
                usage: wgpu::BufferUsages::INDEX,
            }
        );
      
        let mut s = Self {
            RenderPipeline: render_pipeline,
            TextureAtlas: atlas,
            VertexBuffer: vertex_buffer,
            IndexBuffer: index_buffer,
            UniformBuffer: uniform_buffer,
            UniformBindGroup: uniform_bind_group,
            TextureBindGroup: diffuse_bind_group,
        };

        //validate that the size of a chunk is enough to cover with 2 bytes
        let size = CHUNK_BOUNDS_X * CHUNK_BOUNDS_Y * CHUNK_BOUNDS_Z;
        if size > u16::MAX as u32 {
            panic!("Error! Cannot create world renderer because the size of a chunk ({} blocks) is to large for the 2 byte unsigned int ceiling ({})", size, u16::MAX);
        }

        s.Init();
        s
    }

    pub fn Init(&mut self){
   
     

    }

    pub fn Render<'a>(& mut self, chunks: &Vec<Chunk>, renderList: &HashSet<usize>, camera: &Camera, pass: &'a mut wgpu::RenderPass, queue: &wgpu::Queue){
        //Self must outlive render pass. If it doesn't, &self.renderPipeline will be null pointer.
        {
          let ref rp = self.RenderPipeline;
          pass.set_pipeline(rp);
        }

        pass.set_bind_group(0, &self.TextureBindGroup, &[]);
        pass.set_bind_group(1, &self.UniformBindGroup, &[]);

        pass.set_index_buffer(self.IndexBuffer.slice(..), wgpu::IndexFormat::Uint16);
        pass.set_vertex_buffer(0, self.VertexBuffer.slice(..));

        for idx in renderList {
            let chunk = &chunks[*idx];
            //slot = index in &[buffer] array defined up top in New()
            pass.set_vertex_buffer(0, self.VertexBuffer.slice(..));

            queue.write_buffer(&self.VertexBuffer, 0, 
                bytemuck::cast_slice(chunk.Mesh.as_slice())
            );
            queue.write_buffer(&self.UniformBuffer, 0, 
                bytemuck::cast_slice::<Uniform, u8>(&[Uniform {
                    view_proj: camera.GetViewProjection(), 
                    chunk_pos: [chunk.Position.0 as f32, chunk.Position.1 as f32],
                    sprite_dimensions: self.TextureAtlas.CellWidth as f32, 
                    tex_size: [self.TextureAtlas.Image.dimensions().0 as f32, self.TextureAtlas.Image.dimensions().1 as f32],
                    atlas_cols: self.TextureAtlas.Columns as f32
                }])
            );

            pass.draw_indexed(0..(chunk.Mesh.len()as u32 / 4 * 6), 0, 0..1);
        }

    }
}
