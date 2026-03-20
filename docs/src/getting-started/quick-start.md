# 1.2 Quick Start

## Prerequisites

### Ollama (Required for Default Configuration)

The default `vizier init` configuration uses Ollama as the AI provider. Install Ollama:

**macOS:**
```sh
brew install ollama
ollama serve
```

**Linux:**
```sh
curl -fsSL https://ollama.com/install.sh | sh
ollama serve
```

**Windows:**
Download from [ollama.com](https://ollama.com/download/windows)

> **Note:** If you prefer using OpenRouter or other providers instead, you can skip Ollama and configure those providers. See [1.3 Configuration](./configuration.md).

## Initialize Your Workspace

Generate configuration and workspace:

```sh
vizier init
```

This will create a minimal config and sample agent to run in your directory.

## Run Your First Agent

```sh
vizier run
```

## Development Quick Start

For development, clone the repository and use the provided `just` commands:

```sh
# Install dependencies (Rust crates and webui npm packages)
just install

# Run in development mode with hot-reload
just dev

# Build the webui
just build
```

### Development with Python

```sh
# Run with Python feature
just dev-python
```

### Available Just Commands

| Command | Description |
|---------|-------------|
| `just install` | Install all dependencies (Rust crates + webui npm packages) |
| `just dev` | Run in development mode with hot-reload |
| `just dev-python` | Run in development mode with Python support |
| `just run` | Run in release mode |
| `just run-python` | Run in release mode with Python support |
| `just release` | Build release binary |
| `just release-python` | Build release binary with Python support |
| `just tui` | Start the terminal user interface (WIP) |
| `just docker` | Start Docker services (database, etc.) |
| `just build` | Build the webui frontend |

## Next Steps

- Configure your agent: See [1.3 Configuration](./configuration.md)
- Learn about agents: See [3. Agents](../agents/overview.md)
- Explore tools: See [5. Tools](../tools/overview.md)
