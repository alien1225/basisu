# BasisU — PHP Extension

**BasisU** is an experimental PHP extension written in Rust that acts as a wrapper around the native C/C++ [Basis Universal](https://github.com/BinomialLLC/basis_universal) library, exposing its functionality to PHP.

> **Warning: Unstable and not recommended for production use.** This project currently has no automated test suite and has not been sufficiently tested. Use it at your own risk; APIs and behavior may change without notice.

## Requirements

- PHP 8.1–8.5
- Linux with glibc

Currently, only glibc-based Linux environments are supported. musl-based Linux distributions, Windows, and macOS are not supported.

## Installation

The package is available on [Packagist](https://packagist.org/packages/alien1225/basisu) and can be installed using [PIE](https://github.com/php/pie).

### Option 1: Install a prebuilt binary

Precompiled extension binaries are published in [GitHub Releases](https://github.com/alien1225/basisu/releases).

```bash
pie install alien1225/basisu
```

PIE can download a compatible prebuilt binary, so you do not need to compile the extension locally when a suitable binary is available.

### Option 2: Build from source

Building the extension from source requires:

- Rust toolchain, including Cargo
- `libclang-dev`
- A C/C++ compiler and standard build tools
- PHP development tools compatible with your PHP installation

On Debian or Ubuntu, install the system dependencies:

```bash
sudo apt-get update
sudo apt-get install -y build-essential pkg-config libclang-dev
```

Install Rust and Cargo using [rustup](https://rustup.rs/).

Then build the extension using PIE:

```bash
pie build "alien1225/basisu"
```

Make sure PIE is configured to build from source rather than use a prebuilt binary.

## IDE Support

The repository includes [`basisu.stub.php`](basisu.stub.php), which provides PHP declarations for the extension's API.

The stub helps IDEs and static analysis tools provide:

- Code completion
- Type information
- Method and class discovery
- Improved static analysis

The stub only describes the PHP-facing API; it does not implement the extension's functionality.

## Compatibility

| Feature | Status |
|---|---|
| PHP 8.1–8.5 | Supported target versions |
| glibc-based Linux | Supported |
| musl-based Linux | Not currently supported |
| Windows / macOS | Not currently supported |
| Prebuilt binaries | Available through GitHub Releases |
| Build from source | Requires Rust, Cargo, `libclang-dev`, and build tools |
| Automated test suite | Not available |
| Production readiness | Experimental; not recommended for production |

## Project Status

This project is experimental and should be considered unstable.

There is currently no automated test suite to verify correctness across PHP versions, image inputs, texture formats, or supported environments. Successful compilation does not guarantee correct behavior in every use case.

Use the extension for experimentation and development, and validate its behavior thoroughly before relying on it in any critical workload.

## Example
```php
<?php

use Bu\Encoder;
use Bu\Enums\MipmapMode;
use Bu\Enums\TextureContainer;
use Bu\Enums\TextureFormat;
use Bu\EncoderParams;

$encoderParams = new EncoderParams(
    TextureContainer::Basis,
    TextureFormat::Etc1S,
    85, // Quality level must be between 1 and 100. An exception is thrown if the value is out of range.
    2 // Effort level must be between 0 and 10. An exception is thrown if the value is out of range.
);

/**
 * Other optional parameters of the `EncoderParams` class:
 * 
 * object(Bu\EncoderParams)#1 (10) {
 *  ["qualityLevel"]=> int(85)
 *  ["effortLevel"]=> int(2)
 *  ["textureFormat"]=> enum(Bu\Enums\TextureFormat::Etc1S)
 *  ["textureContainer"]=> enum(Bu\Enums\TextureContainer::Basis)
 *  ["mipmapMode"]=> enum(Bu\Enums\MipmapMode::None)
 *  ["debugOutput"]=> bool(false)
 *  ["threadedCompression"]=> bool(false)
 *  ["srgb"]=> bool(true)
 *  ["ktx2UastcZstd"]=> bool(true)
 *  ["uastcRdoOrDctQuality"]=> float(0)
 * }
 */
var_dump($encoderParams);

$encoderParams->setThreadedCompression(true);
$encoderParams->setTextureFormat(TextureFormat::UastcLdr4X4);
$encoderParams->setTextureContainer(TextureContainer::Ktx2);
$encoderParams->setSrgb(true);
$encoderParams->setThreadedCompression(true);
$encoderParams->setDebugOutput(true);
$encoderParams->setKtx2UastcZstd(true);
$encoderParams->setMipmapMode(MipmapMode::Clamp);

$encoder = new Encoder($encoderParams);
$encoder->compressFileIntoFile(__DIR__ . "/target.png", __DIR__ . "/compressed.ktx2");
```

## Links

- **Source code:** https://github.com/alien1225/basisu
- **Packagist:** https://packagist.org/packages/alien1225/basisu
- **Prebuilt binaries:** https://github.com/alien1225/basisu/releases
- **Basis Universal (upstream C/C++ library):** https://github.com/BinomialLLC/basis_universal
- **Rust bindings:** https://crates.io/crates/basisu_c_sys
- **PIE:** https://github.com/php/pie

## License

This project is licensed under the **MIT License**.
