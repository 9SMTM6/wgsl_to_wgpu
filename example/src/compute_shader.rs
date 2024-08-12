// File automatically generated by build.rs.
// Changes made to this file will not be saved.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, encase::ShaderType)]
pub struct Uniforms {
    pub color_rgb: glam::Vec3,
}
pub mod bind_groups {
    #[derive(Debug)]
    pub struct BindGroup0(wgpu::BindGroup);
    #[derive(Debug)]
    pub struct BindGroupLayout0<'a> {
        pub uniforms: wgpu::BufferBinding<'a>,
    }
    const LAYOUT_DESCRIPTOR0: wgpu::BindGroupLayoutDescriptor = wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage {
                        read_only: false,
                    },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    };
    impl BindGroup0 {
        pub fn get_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
            device.create_bind_group_layout(&LAYOUT_DESCRIPTOR0)
        }
        /// This gets you a bindgroup with customizable layout.
        ///
        /// That allows you to reuse the same bindgroup in different shaders, and allows for better interoperability, since it returns a type-erased wgpu type.
        ///
        /// However this will sidestep some of the safeties provided if you use the [`BindGroups::set`] method instead.
        pub fn unsafe_get_bind_group(
            device: &wgpu::Device,
            bindings: BindGroupLayout0,
            layout: &wgpu::BindGroupLayoutDescriptor,
        ) -> wgpu::BindGroup {
            let bind_group_layout = device.create_bind_group_layout(layout);
            device
                .create_bind_group(
                    &wgpu::BindGroupDescriptor {
                        layout: &bind_group_layout,
                        entries: &[
                            wgpu::BindGroupEntry {
                                binding: 0,
                                resource: wgpu::BindingResource::Buffer(bindings.uniforms),
                            },
                        ],
                        label: None,
                    },
                )
        }
        pub fn from_bindings(device: &wgpu::Device, bindings: BindGroupLayout0) -> Self {
            let bind_group = Self::unsafe_get_bind_group(
                device,
                bindings,
                &LAYOUT_DESCRIPTOR0,
            );
            Self(bind_group)
        }
        pub fn set<'a>(&'a self, render_pass: &mut wgpu::ComputePass<'a>) {
            render_pass.set_bind_group(0, &self.0, &[]);
        }
    }
    #[derive(Debug, Copy, Clone)]
    pub struct BindGroups<'a> {
        pub bind_group0: &'a BindGroup0,
    }
    impl<'a> BindGroups<'a> {
        pub fn set(&self, pass: &mut wgpu::ComputePass<'a>) {
            self.bind_group0.set(pass);
        }
    }
}
pub fn set_bind_groups<'a>(
    pass: &mut wgpu::ComputePass<'a>,
    bind_group0: &'a bind_groups::BindGroup0,
) {
    bind_group0.set(pass);
}
pub mod compute {
    pub const MAIN_WORKGROUP_SIZE: [u32; 3] = [1, 1, 1];
    pub fn create_main_pipeline(device: &wgpu::Device) -> wgpu::ComputePipeline {
        let module = super::create_shader_module(device);
        let layout = super::create_pipeline_layout(device);
        device
            .create_compute_pipeline(
                &wgpu::ComputePipelineDescriptor {
                    label: Some("Compute Pipeline main"),
                    layout: Some(&layout),
                    module: &module,
                    entry_point: "main",
                    compilation_options: Default::default(),
                    cache: Default::default(),
                },
            )
    }
}
pub const ENTRY_MAIN: &str = "main";
pub fn create_shader_module(device: &wgpu::Device) -> wgpu::ShaderModule {
    let source = std::borrow::Cow::Borrowed(include_str!("compute_shader.wgsl"));
    device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(source),
        })
}
pub fn create_pipeline_layout(device: &wgpu::Device) -> wgpu::PipelineLayout {
    device
        .create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[
                    &bind_groups::BindGroup0::get_bind_group_layout(device),
                ],
                push_constant_ranges: &[],
            },
        )
}
