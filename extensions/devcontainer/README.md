# Dev Container Extension for Zed

This extension enables remote development in containers using the [Dev Container specification](https://containers.dev/), similar to Visual Studio Code's Dev Containers feature.

## Overview

The Dev Container extension allows you to:
- Open any project inside a Docker or Podman container
- Use containers as a full-featured development environment
- Automatically configure your development environment using a simple JSON file
- Leverage Zed's remote development capabilities to work seamlessly with containers

## Features

- **Parse devcontainer.json**: Automatically reads and parses `.devcontainer/devcontainer.json` configuration files
- **Container Management**: Create and manage development containers using Docker or Podman
- **Remote Development**: Seamlessly connect to containers using Zed's built-in remote development capabilities
- **Slash Commands**: Easy-to-use commands in the Assistant panel
- **Multiple Container Runtimes**: Support for both Docker and Podman

## Prerequisites

- **Docker or Podman** installed and running on your system
  - Docker: https://docs.docker.com/get-docker/
  - Podman: https://podman.io/getting-started/installation
- **Zed editor** with remote development support (v0.159+)

## Installation

### From Zed Extensions (Future)

Once published, you'll be able to install this extension directly from Zed's extension marketplace.

### As a Dev Extension (Current)

1. Clone the Zed repository
2. Navigate to `extensions/devcontainer`
3. Open Zed and run "Install Dev Extension" from the Command Palette
4. Select the `extensions/devcontainer` directory

## Usage

### Opening a Project in a Devcontainer

1. Create a `.devcontainer/devcontainer.json` file in your project root:
   ```json
   {
     "name": "My Project",
     "image": "node:20-bullseye",
     "workspaceFolder": "/workspace",
     "forwardPorts": [3000, 8080],
     "postCreateCommand": "npm install"
   }
   ```

2. Open the Zed Assistant panel (Cmd/Ctrl+?)

3. Use the `/devcontainer-open` slash command with your project path:
   ```
   /devcontainer-open /path/to/your/project
   ```

4. The extension will:
   - Find and parse your `devcontainer.json`
   - Create a new container based on your configuration
   - Run any post-create commands
   - Provide instructions to connect via Zed's remote development

5. Follow the provided instructions to connect to the container using Zed's remote development feature

### Attaching to an Existing Container

If you have an existing container you want to work with:

1. Open the Zed Assistant panel
2. Use the `/devcontainer-attach` slash command:
   ```
   /devcontainer-attach
   ```
3. Select a running container from the autocomplete suggestions
4. Follow the provided instructions to connect

### Rebuilding a Devcontainer

To rebuild your devcontainer with updated configuration:

1. Open the Zed Assistant panel
2. Use the `/devcontainer-rebuild` slash command:
   ```
   /devcontainer-rebuild
   ```
3. Follow the instructions to recreate your container

## Devcontainer Configuration

Create a `.devcontainer/devcontainer.json` file in your project root. Here's a complete example:

```json
{
  "name": "My Development Container",
  "image": "mcr.microsoft.com/devcontainers/typescript-node:1-20-bullseye",
  "workspaceFolder": "/workspace",
  "workspaceMount": "type=bind,source=${localWorkspaceFolder},target=/workspace",
  "forwardPorts": [3000, 8080],
  "mounts": [
    "type=bind,source=/host/path,target=/container/path"
  ],
  "runArgs": ["--privileged"],
  "postCreateCommand": "npm install && npm run setup",
  "postStartCommand": "npm run dev",
  "remoteUser": "node"
}
```

### Supported Configuration Options

| Option | Type | Description |
|--------|------|-------------|
| `name` | string | Container name (optional) |
| `image` | string | Docker image to use |
| `dockerfile` | string | Path to Dockerfile (alternative to image)* |
| `workspaceFolder` | string | Path inside container where project will be mounted |
| `workspaceMount` | string | Custom mount configuration |
| `mounts` | array | Additional mount points |
| `runArgs` | array | Additional arguments for `docker run` |
| `postCreateCommand` | string/array | Command to run after container creation |
| `postStartCommand` | string/array | Command to run when container starts |
| `postAttachCommand` | string/array | Command to run when attaching to container |
| `forwardPorts` | array | Ports to forward from container |
| `remoteUser` | string | User to connect as |

*Note: Building from Dockerfile is not yet fully automated. Build the image manually first and reference it in the `image` field.

## Examples

The `examples/` directory contains sample configurations for:
- Node.js/TypeScript development
- Python development
- Rust development

See [examples/README.md](examples/README.md) for more details.

## How It Works

1. **Configuration Parsing**: The extension reads your `devcontainer.json` file and parses the configuration
2. **Container Runtime Detection**: Automatically detects whether Docker or Podman is available
3. **Container Creation**: Creates a new container with the specified image, mounts, and configuration
4. **Post-Creation Setup**: Runs any specified post-create commands inside the container
5. **Connection Instructions**: Provides the necessary commands to connect Zed to the container using its remote development feature

## Limitations

- Building from Dockerfile is not yet fully automated (build the image manually first)
- Some advanced devcontainer features may not be supported yet
- Container lifecycle management is basic (no automatic cleanup)
- Requires manual connection via Zed's remote development feature

## Troubleshooting

### Docker/Podman Not Found

Ensure Docker or Podman is installed and running:
```bash
docker --version
# or
podman --version
```

### Container Creation Fails

1. Check the Zed log: Command Palette → "Open Log"
2. Verify your devcontainer.json is valid JSON
3. Ensure the specified image exists or can be pulled

### Cannot Connect to Container

1. Verify the container is running: `docker ps` or `podman ps`
2. Check that the container has a shell available (`sh` or `bash`)
3. Ensure Zed's remote development feature is working

### Extension Not Loading

1. Reinstall the dev extension
2. Check for compilation errors: `cargo build --target wasm32-wasip2 --release`
3. Restart Zed with `--foreground` flag to see debug output

For detailed testing instructions, see [TESTING.md](TESTING.md).

## Contributing

Contributions are welcome! This extension is part of the Zed repository.

### Development

1. Build the extension:
   ```bash
   cd extensions/devcontainer
   cargo build --target wasm32-wasip2 --release
   ```

2. Install as a dev extension in Zed

3. Make changes and rebuild

4. Test your changes following [TESTING.md](TESTING.md)

## Resources

- [Dev Container Specification](https://containers.dev/)
- [Zed Remote Development Documentation](https://zed.dev/docs/remote-development)
- [Zed Extension Development Guide](https://zed.dev/docs/extensions)

## License

Apache-2.0
