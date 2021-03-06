# ![trash](logo.svg)

Move files and folders to the trash

<!-- omit in toc -->
## Table of contents

- [Introduction](#introduction)
- [Examples](#examples)
- [Install trash](#install-trash)
  - [Pre-built binaries](#pre-built-binaries)
  - [From source](#from-source)

## Introduction

`trash` is a command-line application for moving files and folders into the trash. A safer alternative to `rm`.

## Examples

```sh
trash README.md # move README.md to the trash
trash "*.md" # move all files ending in .md in the current direcory to the trash
trash src install.sh # move src folder and install.sh file to the trash
```

## Install trash

### Pre-built binaries

Trash works across all major platforms. You can directly install the binary from GitHub:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/JakeChampion/trash/master/install.sh | sh
```

If you would rather not run a script, you can download the binary from the [Releases](https://github.com/JakeChampion/trash/releases) page and add it to your `PATH` environment variable.

### From source

If you have Rust and Cargo install you can install Trash from source:

```sh
cargo install trash-it
```
