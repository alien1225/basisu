use crate::enums::{MipmapMode, TextureContainer, TextureFormat};
use basisu_c_sys::{common, BasisTextureFormat};
use basisu_c_sys::extra::BasisuEncoderParams;
use ext_php_rs::exception::PhpResult;
use ext_php_rs::{php_class, php_impl};

#[php_class]
#[php(name = "Bu\\EncoderParams")]
#[derive(Copy, Clone)]
pub struct EncoderParams {
    #[php(prop)]
    quality_level: i32,
    #[php(prop)]
    effort_level: i32,
    #[php(prop)]
    texture_format: TextureFormat,
    #[php(prop)]
    texture_container: TextureContainer,
    #[php(prop)]
    mipmap_mode: MipmapMode,
    #[php(prop)]
    debug_output: bool,
    #[php(prop)]
    threaded_compression: bool,
    #[php(prop)]
    srgb: bool,
    #[php(prop)]
    ktx2_uastc_zstd: bool,
    #[php(prop)]
    uastc_rdo_or_dct_quality: f32,
}

impl Default for EncoderParams {
    fn default() -> Self {
        Self {
            quality_level: 75,
            effort_level: 2,
            texture_format: TextureFormat::UastcLdr4x4,
            texture_container: TextureContainer::Ktx2,
            mipmap_mode: MipmapMode::None,
            debug_output: false,
            threaded_compression: false,
            srgb: true,
            ktx2_uastc_zstd: true,
            uastc_rdo_or_dct_quality: 0.0,
        }
    }
}

#[php_impl]
impl EncoderParams {
    pub fn __construct(
        texture_container: TextureContainer,
        texture_format: TextureFormat,
        quality_level: i32,
        effort_level: i32,
    ) -> PhpResult<Self> {
        assert_quality_level(quality_level)?;
        assert_effort_level(effort_level)?;

        PhpResult::Ok(Self {
            texture_container,
            texture_format,
            quality_level,
            effort_level,
            ..EncoderParams::default()
        })
    }

    pub fn set_quality_level(&mut self, quality_level: i32) -> PhpResult<()> {
        assert_quality_level(quality_level)?;

        self.quality_level = quality_level;

        Ok(())
    }

    pub fn get_quality_level(&self) -> i32 {
        self.quality_level
    }

    pub fn set_effort_level(&mut self, effort_level: i32) -> PhpResult<()> {
        assert_effort_level(effort_level)?;

        self.effort_level = effort_level;

        Ok(())
    }

    pub fn get_effort_level(&self) -> i32 {
        self.effort_level
    }

    pub fn set_texture_format(&mut self, texture_format: TextureFormat) {
        self.texture_format = texture_format;
    }

    pub fn get_texture_format(&self) -> TextureFormat {
        self.texture_format
    }

    pub fn set_texture_container(&mut self, texture_container: TextureContainer) {
        self.texture_container = texture_container;
    }

    pub fn get_texture_container(&self) -> TextureContainer {
        self.texture_container
    }

    pub fn set_mipmap_mode(&mut self, mipmap_mode: MipmapMode) {
        self.mipmap_mode = mipmap_mode;
    }

    pub fn get_mipmap_mode(&self) -> MipmapMode {
        self.mipmap_mode
    }

    pub fn set_debug_output(&mut self, debug_output: bool) {
        self.debug_output = debug_output;
    }

    pub fn get_debug_output(&self) -> bool {
        self.debug_output
    }

    pub fn set_threaded_compression(&mut self, threaded_compression: bool) {
        self.threaded_compression = threaded_compression;
    }

    pub fn get_threaded_compression(&self) -> bool {
        self.threaded_compression
    }

    pub fn set_srgb(&mut self, srgb: bool) {
        self.srgb = srgb;
    }

    pub fn get_srgb(&self) -> bool {
        self.srgb
    }

    pub fn set_ktx2_uastc_zstd(&mut self, ktx2_uastc_zstd: bool) {
        self.ktx2_uastc_zstd = ktx2_uastc_zstd;
    }

    pub fn get_ktx2_uastc_zstd(&self) -> bool {
        self.ktx2_uastc_zstd
    }

    pub fn set_uastc_rdo_or_dct_quality(&mut self, uastc_rdo_or_dct_quality: f32) {
        self.uastc_rdo_or_dct_quality = uastc_rdo_or_dct_quality;
    }

    pub fn get_uastc_rdo_or_dct_quality(&self) -> f32 {
        self.uastc_rdo_or_dct_quality
    }
}

impl From<&EncoderParams> for BasisuEncoderParams {
    fn from(params: &EncoderParams) -> Self {
        let mut flags_and_quality = 0_u64;

        if let TextureContainer::Ktx2 = params.texture_container {
            flags_and_quality |= common::BU_COMP_FLAGS_KTX2_OUTPUT
        }

        if params.ktx2_uastc_zstd {
            flags_and_quality |= common::BU_COMP_FLAGS_KTX2_UASTC_ZSTD
        }

        if params.srgb {
            flags_and_quality |= common::BU_COMP_FLAGS_SRGB
        }

        if params.threaded_compression {
           flags_and_quality |= common::BU_COMP_FLAGS_THREADED;
        }

        if params.debug_output {
            flags_and_quality |= common::BU_COMP_FLAGS_DEBUG_OUTPUT;
        }

        flags_and_quality |= match params.mipmap_mode {
            MipmapMode::Wrap => common::BU_COMP_FLAGS_GEN_MIPS_WRAP,
            MipmapMode::Clamp => common::BU_COMP_FLAGS_GEN_MIPS_CLAMP,
            MipmapMode::None => 0
        };

        let is_hdr = is_hdr_tf(params.texture_format);

        let quality_level = if is_hdr {
            0
        } else {
            params.quality_level
        };

        let rdo_quality = if is_hdr {
            0_f32
        } else {
            params.uastc_rdo_or_dct_quality
        };

        Self {
            basis_tex_format: BasisTextureFormat::from(&params.texture_format),
            quality_level,
            effort_level: params.effort_level,
            flags_and_quality,
            low_level_uastc_rdo_or_dct_quality: rdo_quality,
        }
    }
}

pub fn is_hdr_tf(tf: TextureFormat) -> bool {
    matches!(
        tf,
        TextureFormat::UastcHdr4x4
            | TextureFormat::UastcHdr6x6
            | TextureFormat::AstcHdr6x6
    )
}

fn assert_quality_level(quality_level: i32) -> Result<(), String> {
    (quality_level <= common::BU_QUALITY_MAX && quality_level >= common::BU_QUALITY_MIN).ok_or(
        format!("The value `quality level` must be between {} and {}", common::BU_QUALITY_MIN, common::BU_QUALITY_MAX)
    )?;

    Ok(())
}

fn assert_effort_level(effort_level: i32) -> Result<(), String> {
    (effort_level <= common::BU_EFFORT_MAX && effort_level >= common::BU_EFFORT_MIN).ok_or(
        format!("The value `effort level` must be between {} and {}", common::BU_EFFORT_MIN, common::BU_EFFORT_MAX)
    )?;

    Ok(())
}
