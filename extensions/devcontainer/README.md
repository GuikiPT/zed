# Dev Container Extension for Zed

This extension enables remote development in containers using the [Dev Container specification](https://containers.dev/), similar to Visual Studio Code's Dev Containers feature.

## Features

- **Parse devcontainer.json**: Automatically reads and parses `.devcontainer/devcontainer.json` configuration files
- **Container Management**: Create and manage development containers using Docker or Podman
- **Remote Development**: Seamlessly connect to containers using Zed's built-in remote development capabilities
- **Slash Commands**: Easy-to-use commands in the Assistant panel

## Prerequisites

- Docker or Podman installed and running on your system
- Zed editor with remote development support (v0.159+)

## Usage

### Opening a Project in a Devcontainer

1. Open the Zed Assistant panel
2. Use the `/devcontainer-open` slash command with your project path:
   ```
   /devcontainer-open /path/to/your/project
   ```
3. The extension will:
   - Find and parse your `devcontainer.json`
   - Create a new container based on your configuration
   - Run any post-create commands
   - Provide instructions to connect via Zed's remote development

### Attaching to an Existing Container

1. Open the Zed Assistant panel
2. Use the `/devcontainer-attach` slash command:
   ```
   /devcontainer-attach
   ```
3. Select a running container from the autocomplete suggestions
4. Follow the provided instructions to connect

### Rebuilding a Devcontainer

1. Open the Zed Assistant panel
2. Use the `/devcontainer-rebuild` slash command:
   ```
   /devcontainer-rebuild
   ```

## Devcontainer Configuration

Create a `.devcontainer/devcontainer.json` file in your project root with configuration like:

```json
{
  "name": "My Project",
  "image": "mcr.microsoft.com/devcontainers/typescript-node:1-20-bullseye",
  "workspaceFolder": "/workspace",
  "forwardPorts": [3000, 8080],
  "postCreateCommand": "npm install",
  "remoteUser": "node"
}
```

### Supported Configuration Options

- `name`: Container name
- `image`: Docker image to use
- `dockerfile`: Path to Dockerfile (alternative to image)
- `workspaceFolder`: Path inside container where project will be mounted
- `workspaceMount`: Custom mount configuration
- `mounts`: Additional mount points
- `runArgs`: Additional arguments for `docker run`
- `postCreateCommand`: Command to run after container creation
- `forwardPorts`: Ports to forward from container
- `remoteUser`: User to connect as

## Limitations

- Building from Dockerfile is not yet fully automated (you need to build the image manually first)
- Some advanced devcontainer features may not be supported yet
- Container lifecycle management is basic (no automatic cleanup)

## Troubleshooting

If you encounter issues:

1. Ensure Docker/Podman is running: `docker ps` or `podman ps`
2. Check container logs: `docker logs <container-name>`
3. Verify your devcontainer.json syntax
4. Check Zed logs: Command Palette → "Open Log"

## Contributing

This extension is part of the Zed repository. Contributions are welcome!

## License

Apache-2.0
