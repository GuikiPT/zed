use serde::{Deserialize, Serialize};
use std::fs;
use zed_extension_api::process::Command;
use zed_extension_api::{
    self as zed, serde_json, Result, SlashCommand, SlashCommandArgumentCompletion,
    SlashCommandOutput, SlashCommandOutputSection, Worktree,
};

struct DevcontainerExtension;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct DevcontainerConfig {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    image: Option<String>,
    #[serde(default)]
    dockerfile: Option<String>,
    #[serde(default)]
    context: Option<String>,
    #[serde(default)]
    workspace_folder: Option<String>,
    #[serde(default)]
    workspace_mount: Option<String>,
    #[serde(default)]
    mounts: Vec<String>,
    #[serde(default)]
    run_args: Vec<String>,
    #[serde(default)]
    post_create_command: Option<serde_json::Value>,
    #[serde(default)]
    post_start_command: Option<serde_json::Value>,
    #[serde(default)]
    post_attach_command: Option<serde_json::Value>,
    #[serde(default)]
    forward_ports: Vec<u16>,
    #[serde(default)]
    remote_user: Option<String>,
}

impl DevcontainerExtension {
    fn find_devcontainer_config(worktree_path: &str) -> Result<DevcontainerConfig> {
        let possible_paths = vec![
            format!("{}/.devcontainer/devcontainer.json", worktree_path),
            format!("{}/.devcontainer.json", worktree_path),
        ];

        for path in possible_paths {
            if let Ok(content) = fs::read_to_string(&path) {
                let config: DevcontainerConfig = serde_json::from_str(&content)
                    .map_err(|e| format!("Failed to parse devcontainer.json: {}", e))?;
                return Ok(config);
            }
        }

        Err("No devcontainer.json found in project".to_string())
    }

    fn detect_container_runtime() -> Result<String> {
        if Command::new("docker").arg("--version").output().is_ok() {
            return Ok("docker".to_string());
        }
        
        if Command::new("podman").arg("--version").output().is_ok() {
            return Ok("podman".to_string());
        }

        Err("Neither docker nor podman found. Please install a container runtime.".to_string())
    }

    fn list_running_containers(runtime: &str) -> Result<Vec<(String, String)>> {
        let output = Command::new(runtime)
            .args(["ps", "--format", "{{.ID}}|{{.Names}}"])
            .output()
            .map_err(|e| format!("Failed to list containers: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let containers: Vec<(String, String)> = stdout
            .lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() == 2 {
                    Some((parts[0].to_string(), parts[1].to_string()))
                } else {
                    None
                }
            })
            .collect();

        Ok(containers)
    }

    fn create_container(
        runtime: &str,
        config: &DevcontainerConfig,
        project_path: &str,
    ) -> Result<String> {
        let container_name = config
            .name
            .as_ref()
            .map(|n| format!("zed-devcontainer-{}", n.replace(' ', "-")))
            .unwrap_or_else(|| "zed-devcontainer".to_string());

        let mut args = vec![
            "run".to_string(),
            "-d".to_string(),
            "--name".to_string(),
            container_name.clone(),
        ];

        let default_mount = format!(
            "type=bind,source={},target=/workspace",
            project_path
        );
        let workspace_mount = config
            .workspace_mount
            .as_deref()
            .unwrap_or(&default_mount);
        args.push("--mount".to_string());
        args.push(workspace_mount.to_string());

        for mount in &config.mounts {
            args.push("--mount".to_string());
            args.push(mount.clone());
        }

        for port in &config.forward_ports {
            args.push("-p".to_string());
            args.push(format!("{}:{}", port, port));
        }

        args.extend(config.run_args.clone());

        if let Some(image) = &config.image {
            args.push(image.clone());
        } else if let Some(dockerfile) = &config.dockerfile {
            return Err(format!(
                "Building from Dockerfile not yet implemented. Please build the image manually and specify it in 'image' field. Dockerfile: {}",
                dockerfile
            ));
        } else {
            return Err("No image or dockerfile specified in devcontainer.json".to_string());
        }

        args.push("sleep".to_string());
        args.push("infinity".to_string());

        let output = Command::new(runtime)
            .args(&args)
            .output()
            .map_err(|e| format!("Failed to create container: {}", e))?;

        if output.status != Some(0) {
            return Err(format!(
                "Failed to create container: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(container_name)
    }

    fn exec_in_container(
        runtime: &str,
        container_name: &str,
        command: &str,
    ) -> Result<String> {
        let output = Command::new(runtime)
            .args(["exec", container_name, "sh", "-c", command])
            .output()
            .map_err(|e| format!("Failed to execute command in container: {}", e))?;

        if output.status != Some(0) {
            return Err(format!(
                "Command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    fn run_post_create_commands(
        runtime: &str,
        container_name: &str,
        config: &DevcontainerConfig,
    ) -> Result<()> {
        if let Some(cmd) = &config.post_create_command {
            let command_str = match cmd {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Array(arr) => arr
                    .iter()
                    .filter_map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(" "),
                _ => return Ok(()),
            };

            Self::exec_in_container(runtime, container_name, &command_str)?;
        }
        Ok(())
    }
}

impl zed::Extension for DevcontainerExtension {
    fn new() -> Self {
        Self
    }

    fn run_slash_command(
        &self,
        command: SlashCommand,
        args: Vec<String>,
        worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        match command.name.as_str() {
            "devcontainer-open" => {
                if args.is_empty() {
                    return Err("Please provide a project path".to_string());
                }

                let project_path = args.join(" ");
                
                let config = Self::find_devcontainer_config(&project_path)?;
                let runtime = Self::detect_container_runtime()?;

                let mut output_text = String::new();
                output_text.push_str(&format!(
                    "Opening devcontainer for project: {}\n\n",
                    project_path
                ));
                output_text.push_str(&format!("Container runtime: {}\n", runtime));
                
                if let Some(name) = &config.name {
                    output_text.push_str(&format!("Devcontainer name: {}\n", name));
                }
                if let Some(image) = &config.image {
                    output_text.push_str(&format!("Using image: {}\n", image));
                }

                output_text.push_str("\nCreating container...\n");
                let container_name = Self::create_container(&runtime, &config, &project_path)?;
                output_text.push_str(&format!("Container created: {}\n", container_name));

                output_text.push_str("\nRunning post-create commands...\n");
                if let Err(e) = Self::run_post_create_commands(&runtime, &container_name, &config) {
                    output_text.push_str(&format!("Warning: Post-create command failed: {}\n", e));
                }

                output_text.push_str("\n✓ Container is ready!\n\n");
                output_text.push_str(&format!(
                    "To connect to this container, use Zed's remote development feature:\n\
                     1. Open Command Palette (Cmd/Ctrl+Shift+P)\n\
                     2. Run 'projects: Open Remote'\n\
                     3. Connect using: ssh root@localhost -o ProxyCommand=\"{} exec -i {} sh\"\n\n\
                     Or use the /devcontainer-attach command with the container name.",
                    runtime, container_name
                ));

                Ok(SlashCommandOutput {
                    sections: vec![SlashCommandOutputSection {
                        range: (0..output_text.len()).into(),
                        label: "Devcontainer Open".to_string(),
                    }],
                    text: output_text,
                })
            }
            "devcontainer-rebuild" => {
                let worktree = worktree.ok_or("No worktree available")?;
                let worktree_path = worktree.root_path();

                let _config = Self::find_devcontainer_config(&worktree_path)?;
                let _runtime = Self::detect_container_runtime()?;

                let mut output_text = String::new();
                output_text.push_str("Rebuilding devcontainer...\n\n");

                output_text.push_str("This feature will:\n");
                output_text.push_str("1. Stop the current container\n");
                output_text.push_str("2. Remove the container\n");
                output_text.push_str("3. Create a new container with updated configuration\n\n");
                output_text.push_str(
                    "Note: Manual rebuild is required. Use /devcontainer-open to create a new container.\n",
                );

                Ok(SlashCommandOutput {
                    sections: vec![SlashCommandOutputSection {
                        range: (0..output_text.len()).into(),
                        label: "Devcontainer Rebuild".to_string(),
                    }],
                    text: output_text,
                })
            }
            "devcontainer-attach" => {
                if args.is_empty() {
                    return Err("Please provide a container name or ID".to_string());
                }

                let container_id = args.join(" ");
                let runtime = Self::detect_container_runtime()?;

                let output = Command::new(&runtime)
                    .args(["inspect", "--format", "{{.State.Running}}", &container_id])
                    .output()
                    .map_err(|e| format!("Failed to inspect container: {}", e))?;

                let is_running = String::from_utf8_lossy(&output.stdout).trim() == "true";

                let mut output_text = String::new();
                output_text.push_str(&format!("Container: {}\n", container_id));
                output_text.push_str(&format!("Status: {}\n\n", if is_running { "Running" } else { "Stopped" }));

                if !is_running {
                    output_text.push_str("Container is not running. Starting it...\n");
                    Command::new(&runtime)
                        .args(["start", &container_id])
                        .output()
                        .map_err(|e| format!("Failed to start container: {}", e))?;
                    output_text.push_str("Container started.\n\n");
                }

                output_text.push_str(&format!(
                    "To connect to this container using Zed's remote development:\n\
                     1. Open Command Palette (Cmd/Ctrl+P)\n\
                     2. Run 'projects: Open Remote'\n\
                     3. Connect using: ssh root@localhost -o ProxyCommand=\"{} exec -i {} sh\"\n",
                    runtime, container_id
                ));

                Ok(SlashCommandOutput {
                    sections: vec![SlashCommandOutputSection {
                        range: (0..output_text.len()).into(),
                        label: "Devcontainer Attach".to_string(),
                    }],
                    text: output_text,
                })
            }
            command => Err(format!("unknown slash command: \"{}\"", command)),
        }
    }

    fn complete_slash_command_argument(
        &self,
        command: SlashCommand,
        _args: Vec<String>,
    ) -> Result<Vec<SlashCommandArgumentCompletion>, String> {
        match command.name.as_str() {
            "devcontainer-open" => Ok(vec![]),
            "devcontainer-rebuild" => Ok(vec![]),
            "devcontainer-attach" => {
                let runtime = Self::detect_container_runtime()?;
                let containers = Self::list_running_containers(&runtime)?;

                Ok(containers
                    .into_iter()
                    .map(|(id, name)| SlashCommandArgumentCompletion {
                        label: format!("{} ({})", name, id),
                        new_text: id,
                        run_command: true,
                    })
                    .collect())
            }
            command => Err(format!("unknown slash command: \"{}\"", command)),
        }
    }
}

zed::register_extension!(DevcontainerExtension);
