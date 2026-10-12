use crate::params::{is_hdr_tf, EncoderParams};
use basisu_c_sys::extra::types::Extent3d;
use basisu_c_sys::extra::{BasisuEncoder, BasisuEncoderParams, SourceImage, SourceImageFormat};
use ext_php_rs::exception::PhpResult;
use ext_php_rs::{php_class, php_impl};
use image::{DynamicImage, ImageReader};
use std::fs;
use crate::enums::TextureFormat;

#[php_class]
#[php(name = "Bu\\Encoder")]
#[derive(Clone, Copy)]
pub struct Encoder {
    params: EncoderParams,
}

#[php_impl]
impl Encoder {
    pub fn __construct(params: &EncoderParams) -> Self {
        Self {
            params: params.clone(),
        }
    }

    pub fn compress_file_into_file(
        &self,
        in_path: String,
        out_path: String,
        depth_or_array_layers: Option<u32>,
    ) -> PhpResult<()> {
        let image = ImageReader::open(in_path).map_err(|e| e.to_string())?
            .decode().map_err(|e| e.to_string())?;

        let data = decode_img(
            &image,
            self.params.get_texture_format()
        ).ok_or(String::from("Compression error: could not convert image to bytes"))?;

        let width = image.width();
        let height = image.height();
        let format = if is_hdr_tf(self.params.get_texture_format()) {
            SourceImageFormat::Rgba32Float
        } else {
            SourceImageFormat::Rgba8
        };

        let source = SourceImage {
            data: data.as_slice(),
            format,
            size: Extent3d {
                width,
                height,
                depth_or_array_layers: depth_or_array_layers.unwrap_or(1),
            },
        };

        let mut encoder = BasisuEncoder::new();
        encoder.set_image(source).map_err(|e| e.to_string())?;

        let params = BasisuEncoderParams::from(&self.params);

        let ktx2 = encoder.compress(params).map_err(|e| e.to_string())?;
        fs::write(out_path, ktx2).map_err(|e| e.to_string())?;

        Ok(())
    }
}

fn decode_img(texture: &DynamicImage, tf: TextureFormat) -> Option<Vec<u8>> {
    if is_hdr_tf(tf) {
        let image = texture.to_rgba32f();

        Some(bytemuck::cast_slice(image.as_raw()).to_vec())
    } else {
        Some(texture.to_rgba8().into_raw())
    }
}