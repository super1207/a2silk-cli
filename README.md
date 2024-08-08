# Silk Cli

Convert MP3、WAV、OGG(Vorbis)、FLAC to Tencent SILK

## Supported platforms

On Windows, arm64 and x64 are supported. On Linux, only x64 is supported.

## Installation

Go to [Releases](https://github.com/super1207/a2silk-cli/releases) and download it!

<!--
Using [Nix](https://nixos.org/download.html):
```bash
nix-shell -p silk-cli
```
-->

## Usage

```
Usage: a2silk-cli [OPTIONS] --input <INPUT> --output <OUTPUT>

Options:
  -i, --input <INPUT>              Path of the input file
  -o, --output <OUTPUT>            Path of the output file
  -h, --help                       Print help
  -V, --version                    Print version
```

## Thanks

[silk-cli](https://github.com/idranme/silk-cli) for silk

[claxon](https://github.com/ruuda/claxon) for flac

[minimp3-rs](https://github.com/germangb/minimp3-rs) for mp3

[lewton](https://github.com/RustAudio/lewton) for ogg
