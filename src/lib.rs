#![cfg_attr(windows, feature(abi_vectorcall))]

mod encoder;
mod enums;
mod params;

use basisu_c_sys::extra::basisu_encoder_init;
use ext_php_rs::php_module;
use ext_php_rs::prelude::ModuleBuilder;
use crate::encoder::Encoder;
use crate::enums::{MipmapMode, TextureContainer, TextureFormat};
use crate::params::EncoderParams;

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    basisu_encoder_init();

    module
        .name("basisu")
        .version(env!("CARGO_PKG_VERSION"))
        .class::<EncoderParams>()
        .class::<Encoder>()
        .enumeration::<TextureFormat>()
        .enumeration::<TextureContainer>()
        .enumeration::<MipmapMode>()
}
