<?php

namespace Bu {
    class Encoder {
        public function __construct(\Bu\EncoderParams $params) {}

        public function compressFileIntoFile(string $in_path, string $out_path, ?int $depth_or_array_layers = null): void {};
    }

    class EncoderParams {
        public int $qualityLevel;
        public int $effortLevel;
        public \Bu\Enums\TextureFormat $textureFormat;
        public \Bu\Enums\TextureContainer $textureContainer;
        public \Bu\Enums\MipmapMode $mipmapMode;
        public bool $debugOutput;
        public bool $threadedCompression;
        public bool $srgb;
        public bool $ktx2UastcZstd;
        public float $uastcRdoOrDctQuality;

        public function __construct(\Bu\Enums\TextureContainer $texture_container, \Bu\Enums\TextureFormat $texture_format, int $quality_level, int $effort_level) {}

        public function setQualityLevel(int $quality_level): void {}

        public function getQualityLevel(): int {}

        public function setEffortLevel(int $effort_level): void {}

        public function getEffortLevel(): int {}

        public function setTextureFormat(\Bu\Enums\TextureFormat $texture_format): void {}

        public function getTextureFormat(): \Bu\Enums\TextureFormat {}

        public function setTextureContainer(\Bu\Enums\TextureContainer $texture_container): void {}

        public function getTextureContainer(): \Bu\Enums\TextureContainer {}

        public function setMipmapMode(\Bu\Enums\MipmapMode $mipmap_mode): void {}

        public function getMipmapMode(): \Bu\Enums\MipmapMode {}

        public function setDebugOutput(bool $debug_output): void {}

        public function getDebugOutput(): bool {}

        public function setThreadedCompression(bool $threaded_compression): void {}

        public function getThreadedCompression(): bool {}

        public function setSrgb(bool $srgb): void {}

        public function getSrgb(): bool {}

        public function setKtx2UastcZstd(bool $ktx2_uastc_zstd): void {}

        public function getKtx2UastcZstd(): bool {}

        public function setUastcRdoOrDctQuality(float $uastc_rdo_or_dct_quality): void {}

        public function getUastcRdoOrDctQuality(): float {}
    }
}

namespace Bu\Enums {
    enum TextureFormat: int {
        case Etc1S = 0;
        case UastcLdr4X4 = 1;
        case UastcHdr4X4 = 2;
        case AstcHdr6X6 = 3;
        case UastcHdr6X6 = 4;
        case XuastcLdr4X4 = 5;
        case XuastcLdr5X4 = 6;
        case XuastcLdr5X5 = 7;
        case XuastcLdr6X5 = 8;
        case XuastcLdr6X6 = 9;
        case XuastcLdr8X5 = 10;
        case XuastcLdr8X6 = 11;
        case XuastcLdr10X5 = 12;
        case XuastcLdr10X6 = 13;
        case XuastcLdr8X8 = 14;
        case XuastcLdr10X8 = 15;
        case XuastcLdr10X10 = 16;
        case XuastcLdr12X10 = 17;
        case XuastcLdr12X12 = 18;
        case AstcLdr4X4 = 19;
        case AstcLdr5X4 = 20;
        case AstcLdr5X5 = 21;
        case AstcLdr6X5 = 22;
        case AstcLdr6X6 = 23;
        case AstcLdr8X5 = 24;
        case AstcLdr8X6 = 25;
        case AstcLdr10X5 = 26;
        case AstcLdr10X6 = 27;
        case AstcLdr8X8 = 28;
        case AstcLdr10X8 = 29;
        case AstcLdr10X10 = 30;
        case AstcLdr12X10 = 31;
        case AstcLdr12X12 = 32;
        case Xubc7 = 33;
    }

    enum TextureContainer: int {
        case Ktx2 = 0;
        case Basis = 1;
    }

    enum MipmapMode: int {
        case None = 0;
        case Clamp = 1;
        case Wrap = 2;
    }
}