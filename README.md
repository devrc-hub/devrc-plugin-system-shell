# devrc shell plugin


[![Crates.io](https://img.shields.io/crates/v/devrc-plugin-system-shell)](https://crates.io/crates/devrc-plugin-system-shell)
[![Crates.io](https://img.shields.io/crates/d/devrc-plugin-system-shell)](https://crates.io/crates/devrc-plugin-system-shell)
[![CI](https://github.com/devrc-hub/devrc-plugin-system-shell/workflows/CI/badge.svg?branch=master)](https://github.com/devrc-hub/devrc-plugin-system-shell/actions)
[![Security audit](https://github.com/devrc-hub/devrc-plugin-system-shell/workflows/Security%20audit/badge.svg?branch=master)](https://github.com/devrc-hub/devrc-plugin-system-shell/actions)
[![Minimum supported Rust version](https://img.shields.io/badge/rustc-1.48+-brightgreen.svg)](#compile-from-sources)
[![Lines Of Code](https://tokei.rs/b1/github/devrc-hub/devrc-plugin-system-shell?category=code)](#contributing)
[![LICENSE](https://img.shields.io/github/license/devrc-hub/devrc-plugin-system-shell)](https://github.com/devrc-hub/devrc-plugin-system-shell/blob/master/LICENSE)


## Overview

Demo devrc plugin for performing tasks in the system shell.

### Installation


To install `devrc-plugin-system-shell`, you can download a [pre-compiled binary](#binary-releases), or you can [compile it from source](#compile-from-sources).


### Compile from sources

![GitHub](https://img.shields.io/badge/rustc-1.48+-brightgreen.svg) ![GitHub](https://img.shields.io/github/license/devrc-hub/devrc-plugin-system-shell)

Clone the repository and change it to your working directory.

```bash
git clone https://github.com/devrc-hub/devrc-plugin-system-shell.git
cd devrc-plugin-system-shell

rustup update stable
cargo build --release --out-dir /path/to/artifacts
```


### Binary Releases

Binary releases are available in the [github releases page](https://github.com/devrc-hub/devrc-plugin-system-shell/releases) for macOS, and Linux targets.<br>
The following binaries are available for each release:

* x86_64-unknown-linux-musl
* x86_64-unknown-linux-gnu
* x86_64-apple-darwin



## Usage

Add the path to the compiled plugin to your devrc file.

```yaml
devrc_config:
  interpreter:
    runtime: system-shell
    args: ["-c"]
    options:
      shell: zsh
  plugins:
    system-shell: ./path/to/plugin.{so,dylib}


task:
  desc: "system shell "
  run: |
    echo "Hello world"

```
