use bevy::ecs::system::lifetimeless::SRes;
use bevy::ecs::system::SystemParamItem;
use bevy::prelude::{AssetServer, Handle, Image, Shader};
use bevy::reflect::TypeUuid;
use bevy::render::render_asset::{PrepareAssetError, RenderAsset, RenderAssets};
use bevy::render::render_resource::{encase, BindGroup, BindGroupLayout};
use bevy::render::renderer::RenderDevice;
use bevy::sprite::{Material2d, Material2dPipeline};
use wgpu::*;

use crate::plugin::MaskedMesh2dPipeline;

#[derive(TypeUuid, Clone)]
#[uuid = "e66b6c0e-bcac-4128-bdc6-9a3cace5c2fc"]
pub struct MaskAwareMaterial {
    pub mask: Option<Handle<Image>>,
}

pub struct MaskAwareMaterialGPU {
    bind_group: BindGroup,
}

impl Material2d for MaskAwareMaterial {
    fn bind_group(material: &<Self as RenderAsset>::PreparedAsset) -> &BindGroup {
        &material.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        multisampled: false,
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        })
    }
}

impl RenderAsset for MaskAwareMaterial {
    type ExtractedAsset = MaskAwareMaterial;

    type PreparedAsset = MaskAwareMaterialGPU;

    type Param = (
        SRes<RenderDevice>,
        SRes<MaskedMesh2dPipeline>,
        SRes<RenderAssets<Image>>,
    );

    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        material: Self::ExtractedAsset,
        (render_device, pipeline, gpu_images): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        // Ref: bevy-sprite/mesh2d/color_material.rs
        let (texture_view, sampler) = match pipeline
            .mesh2d_pipeline
            .get_image_texture(gpu_images, &material.mask)
        {
            Some(result) => result,
            None => return Err(PrepareAssetError::RetryNextUpdate(material)),
        };
        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &pipeline.material2d_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(texture_view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(sampler),
                },
            ],
        });
        Ok(MaskAwareMaterialGPU { bind_group })
    }
}