use basisu_c_sys::BasisTextureFormat;
use ext_php_rs::php_enum;

#[php_enum]
#[php(name = "Bu\\Enums\\TextureContainer")]
#[derive(Clone, Default, Copy)]
pub enum TextureContainer {
    #[default]
    #[php(value = 0)]
    Ktx2,
    #[php(value = 1)]
    Basis,
}

#[php_enum]
#[php(name = "Bu\\Enums\\MipmapMode")]
#[derive(Clone, Default, Copy)]
pub enum MipmapMode {
    #[default]
    #[php(value = 0)]
    None,
    #[php(value = 1)]
    Clamp,
    #[php(value = 2)]
    Wrap,
}

#[php_enum]
#[php(name = "Bu\\Enums\\TextureFormat")]
#[derive(Clone, Default, Copy, Debug)]
pub enum TextureFormat {
    #[default]
    #[php(value = 0)]
    Etc1s,
    #[php(value = 1)]
    UastcLdr4x4,
    #[php(value = 2)]
    UastcHdr4x4,
    #[php(value = 3)]
    AstcHdr6x6,
    #[php(value = 4)]
    UastcHdr6x6,
    #[php(value = 5)]
    XuastcLdr4x4,
    #[php(value = 6)]
    XuastcLdr5x4,
    #[php(value = 7)]
    XuastcLdr5x5,
    #[php(value = 8)]
    XuastcLdr6x5,
    #[php(value = 9)]
    XuastcLdr6x6,
    #[php(value = 10)]
    XuastcLdr8x5,
    #[php(value = 11)]
    XuastcLdr8x6,
    #[php(value = 12)]
    XuastcLdr10x5,
    #[php(value = 13)]
    XuastcLdr10x6,
    #[php(value = 14)]
    XuastcLdr8x8,
    #[php(value = 15)]
    XuastcLdr10x8,
    #[php(value = 16)]
    XuastcLdr10x10,
    #[php(value = 17)]
    XuastcLdr12x10,
    #[php(value = 18)]
    XuastcLdr12x12,
    #[php(value = 19)]
    AstcLdr4x4,
    #[php(value = 20)]
    AstcLdr5x4,
    #[php(value = 21)]
    AstcLdr5x5,
    #[php(value = 22)]
    AstcLdr6x5,
    #[php(value = 23)]
    AstcLdr6x6,
    #[php(value = 24)]
    AstcLdr8x5,
    #[php(value = 25)]
    AstcLdr8x6,
    #[php(value = 26)]
    AstcLdr10x5,
    #[php(value = 27)]
    AstcLdr10x6,
    #[php(value = 28)]
    AstcLdr8x8,
    #[php(value = 29)]
    AstcLdr10x8,
    #[php(value = 30)]
    AstcLdr10x10,
    #[php(value = 31)]
    AstcLdr12x10,
    #[php(value = 32)]
    AstcLdr12x12,
    #[php(value = 33)]
    Xubc7,
}

impl From<&TextureFormat> for BasisTextureFormat {
    fn from(value: &TextureFormat) -> Self {
        match value {
            TextureFormat::Etc1s => Self::Etc1s,
            TextureFormat::UastcLdr4x4 => Self::UastcLdr4x4,
            TextureFormat::UastcHdr4x4 => Self::UastcHdr4x4,
            TextureFormat::AstcHdr6x6 => Self::AstcHdr6x6,
            TextureFormat::UastcHdr6x6 => Self::UastcHdr6x6,
            TextureFormat::XuastcLdr4x4 => Self::XuastcLdr4x4,
            TextureFormat::XuastcLdr5x4 => Self::XuastcLdr5x4,
            TextureFormat::XuastcLdr5x5 => Self::XuastcLdr5x5,
            TextureFormat::XuastcLdr6x5 => Self::XuastcLdr6x5,
            TextureFormat::XuastcLdr6x6 => Self::XuastcLdr6x6,
            TextureFormat::XuastcLdr8x5 => Self::XuastcLdr8x5,
            TextureFormat::XuastcLdr8x6 => Self::XuastcLdr8x6,
            TextureFormat::XuastcLdr10x5 => Self::XuastcLdr10x5,
            TextureFormat::XuastcLdr10x6 => Self::XuastcLdr10x6,
            TextureFormat::XuastcLdr8x8 => Self::XuastcLdr8x8,
            TextureFormat::XuastcLdr10x8 => Self::XuastcLdr10x8,
            TextureFormat::XuastcLdr10x10 => Self::XuastcLdr10x10,
            TextureFormat::XuastcLdr12x10 => Self::XuastcLdr12x10,
            TextureFormat::XuastcLdr12x12 => Self::XuastcLdr12x12,
            TextureFormat::AstcLdr4x4 => Self::AstcLdr4x4,
            TextureFormat::AstcLdr5x4 => Self::AstcLdr5x4,
            TextureFormat::AstcLdr5x5 => Self::AstcLdr5x5,
            TextureFormat::AstcLdr6x5 => Self::AstcLdr6x5,
            TextureFormat::AstcLdr6x6 => Self::AstcLdr6x6,
            TextureFormat::AstcLdr8x5 => Self::AstcLdr8x5,
            TextureFormat::AstcLdr8x6 => Self::AstcLdr8x6,
            TextureFormat::AstcLdr10x5 => Self::AstcLdr10x5,
            TextureFormat::AstcLdr10x6 => Self::AstcLdr10x6,
            TextureFormat::AstcLdr8x8 => Self::AstcLdr8x8,
            TextureFormat::AstcLdr10x8 => Self::AstcLdr10x8,
            TextureFormat::AstcLdr10x10 => Self::AstcLdr10x10,
            TextureFormat::AstcLdr12x10 => Self::AstcLdr12x10,
            TextureFormat::AstcLdr12x12 => Self::AstcLdr12x12,
            TextureFormat::Xubc7 => Self::Xubc7,
        }
    }
}
