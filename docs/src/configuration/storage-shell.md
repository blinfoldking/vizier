# 1.3.5 Storage & Shell

## `storage`

Configure data persistence backend:

```yaml
storage:
  type: filesystem
```

### Storage Types

| Type | Description |
|------|-------------|
| `filesystem` | Store data in `.vizier/` directory (default) |
| `surreal` | Use SurrealDB for data storage |

## `shell`

Configure the execution environment for shell commands:

```yaml
shell:
  environment: local
  path: "."
```

### Local Environment

```yaml
shell:
  environment: local
  path: "/path/to/working/dir"  # Working directory for shell commands
```

### Docker Environment

```yaml
shell:
  environment: docker
  image:
    source: pull              # Use "pull" or "dockerfile"
    name: "ubuntu:latest"     # Image name (for pull) or "my-image"
  container_name: "vizier"    # Container name
```

For `dockerfile` source:

```yaml
shell:
  environment: docker
  image:
    source: dockerfile
    path: "./Dockerfile"      # Path to Dockerfile
    name: "my-custom-image"   # Image name to build
  container_name: "vizier"
```
