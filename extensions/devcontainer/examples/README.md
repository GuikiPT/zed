# Dev Container Examples

This directory contains example devcontainer.json configurations for various development scenarios.

## Available Examples

### Node.js Development Container
File: `nodejs-devcontainer.json`

A TypeScript/Node.js development environment with Node 20 and common development tools.

### Python Development Container
File: `python-devcontainer.json`

A Python 3.11 development environment with pip and common Python tools.

### Rust Development Container
File: `rust-devcontainer.json`

A Rust development environment with cargo and rustc.

## Usage

To use any of these examples:

1. Copy the desired example file to your project's `.devcontainer/devcontainer.json`
2. Customize it according to your project's needs
3. Use the Zed extension's `/devcontainer-open` command to create and connect to the container

## Customization

You can customize these examples by:

- Adding more ports to `forwardPorts`
- Adding custom `mounts` for additional volumes
- Changing the `postCreateCommand` to run project-specific setup
- Adding `runArgs` for additional Docker runtime arguments
- Specifying a different `remoteUser`

For more information, see the [Dev Container specification](https://containers.dev/).
