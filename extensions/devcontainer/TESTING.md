# Testing the Devcontainer Extension

This document provides instructions for testing the devcontainer extension locally.

## Prerequisites

Before testing the extension, ensure you have:

1. **Docker or Podman** installed and running
   - Docker: https://docs.docker.com/get-docker/
   - Podman: https://podman.io/getting-started/installation

2. **Zed** installed (v0.159+)

3. **Rust toolchain** installed via rustup (for building the extension)

## Building the Extension

1. Navigate to the extension directory:
   ```bash
   cd extensions/devcontainer
   ```

2. Build the extension:
   ```bash
   cargo build --target wasm32-wasip2 --release
   ```

3. The built extension will be at:
   ```
   target/wasm32-wasip2/release/zed_devcontainer_extension.wasm
   ```

## Installing the Extension as a Dev Extension

1. Open Zed
2. Open the Command Palette (Cmd/Ctrl+Shift+P)
3. Run "Install Dev Extension"
4. Select the `extensions/devcontainer` directory
5. The extension will be installed and loaded

## Testing Scenarios

### Scenario 1: Open a Project in a Devcontainer

1. Create a test project with a devcontainer configuration:
   ```bash
   mkdir -p ~/test-devcontainer-project/.devcontainer
   ```

2. Copy one of the example configurations:
   ```bash
   cp extensions/devcontainer/examples/nodejs-devcontainer.json \
      ~/test-devcontainer-project/.devcontainer/devcontainer.json
   ```

3. Create a simple test file:
   ```bash
   echo 'console.log("Hello from devcontainer!");' > ~/test-devcontainer-project/index.js
   ```

4. In Zed, open the Assistant panel (Cmd/Ctrl+?)

5. Use the `/devcontainer-open` slash command:
   ```
   /devcontainer-open ~/test-devcontainer-project
   ```

6. The extension should:
   - Parse the devcontainer.json
   - Detect Docker/Podman
   - Create a new container
   - Provide instructions to connect via Zed's remote development

### Scenario 2: Attach to an Existing Container

1. First, create a container manually:
   ```bash
   docker run -d --name test-container -v ~/test-project:/workspace node:20 sleep infinity
   ```

2. In Zed's Assistant panel, use the `/devcontainer-attach` command:
   ```
   /devcontainer-attach
   ```

3. Select `test-container` from the autocomplete suggestions

4. The extension should provide instructions to connect to the container

### Scenario 3: Rebuild Command

1. Open a project with a devcontainer.json in Zed

2. In the Assistant panel, use:
   ```
   /devcontainer-rebuild
   ```

3. The extension should provide information about rebuilding the container

## Verifying Container Creation

After using `/devcontainer-open`, verify the container was created:

```bash
# Check running containers
docker ps

# Or with Podman
podman ps

# Inspect the container
docker inspect <container-name>

# Execute a command in the container
docker exec -it <container-name> bash
```

## Testing Different Configurations

Test the extension with various devcontainer configurations:

1. **Different base images:**
   - Node.js: `node:20-bullseye`
   - Python: `python:3.11-bullseye`
   - Rust: `rust:1-bullseye`

2. **Port forwarding:**
   ```json
   {
     "forwardPorts": [3000, 8080, 5432]
   }
   ```

3. **Custom mounts:**
   ```json
   {
     "mounts": [
       "type=bind,source=/host/path,target=/container/path"
     ]
   }
   ```

4. **Post-create commands:**
   ```json
   {
     "postCreateCommand": "npm install && npm run setup"
   }
   ```

## Troubleshooting

### Container not created

1. Check if Docker/Podman is running:
   ```bash
   docker ps
   # or
   podman ps
   ```

2. Check Zed logs (Cmd/Ctrl+Shift+P → "Open Log") for error messages

3. Verify the devcontainer.json is valid JSON

### Extension not loading

1. Ensure the extension is built: `cargo build --target wasm32-wasip2 --release`
2. Reinstall the dev extension
3. Restart Zed with `--foreground` flag to see debug output:
   ```bash
   zed --foreground
   ```

### Cannot connect to container

1. Verify the container is running: `docker ps`
2. Check SSH is available in Zed
3. Ensure the container has `sh` or `bash` available

## Manual Testing Checklist

- [ ] Extension installs successfully as a dev extension
- [ ] `/devcontainer-open` command appears in slash command completions
- [ ] `/devcontainer-attach` command appears with container autocomplete
- [ ] `/devcontainer-rebuild` command appears
- [ ] Container is created with correct configuration
- [ ] Post-create commands execute successfully
- [ ] Port forwarding is configured correctly
- [ ] Mounts are set up properly
- [ ] Container can be attached to existing container
- [ ] Error messages are clear and helpful
- [ ] Works with both Docker and Podman

## Automated Testing

Currently, there are no automated tests for the extension. Future work could include:

- Unit tests for configuration parsing
- Integration tests for container creation
- E2E tests with actual Docker/Podman

## Cleanup

After testing, clean up created containers:

```bash
# List containers
docker ps -a | grep zed-devcontainer

# Remove a specific container
docker rm -f <container-name>

# Remove all test containers
docker ps -a | grep zed-devcontainer | awk '{print $1}' | xargs docker rm -f
```
