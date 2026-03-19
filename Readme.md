# Vizier

> **Disclaimer:** this project currently on high-speed development mode; Readmes and Documentations may not properly updated yet

> 21st Century Digital Steward; Right-hand agent for you majesty

Vizier is a Rust-based AI agent framework that provides a unified interface for AI assistants across multiple communication channels (Discord, HTTP, etc.) with memory, tool usage, and extensible architecture.

## Features

- **Multi-Channel Support**: Connect to Discord, HTTP (REST API & WebSocket), and WebUI
- **AI Model Integration**: Support for multiple AI providers (DeepSeek, OpenRouter, Ollama, etc.)
- **Memory System**: Session-based short-term memory, configurable recall depth, and vector-based long-term memory
- **Tool System**: Extensible tool framework including CLI access, web search (Brave Search), Python interpreter (optional), scheduler (cron & one-time tasks), vector memory, and workspace document management
- **Scheduler**: Built-in task scheduler for automated agent execution
- **WebUI**: Modern React-based web interface for interaction and management
- **TUI Interface**: Built-in terminal user interface for local interaction (WIP)
- **Configuration Driven**: Flexible configuration via YAML files with environment-specific overrides

## Installation and Configuration

### Prerequisites

- [Rust and Cargo](https://rust-lang.org/) installed
- **Python 3.9 or higher** (Optional) - Only required if you want to use the Python interpreter tool

#### Python Support (Optional)

Vizier supports Python as an optional feature. By default, Vizier includes Python interpreter support. You can:

- **Use Python interpreter tool**: Install Python 3.9+ (see below)
- **Build without Python**: Use `--no-default-features` flag (see [Building without Python](#building-without-python))

If you want to use the Python interpreter tool, install Python 3.9+:

**macOS:**
```sh
brew install python@3.9
```

**Ubuntu/Debian:**
```sh
sudo apt-get install python3.9 python3.9-dev
```

**Windows:**
Download from [python.org](https://www.python.org/downloads/)

### Quick Start

1. Install the `vizier` binary:
   ```sh
   cargo install vizier
   # Or using cargo-binstall (faster)
   cargo binstall vizier
   ```

2. Generate your initial configuration and workspace:
   ```sh
   vizier init
   ```
   This will create a `.vizier` directory with a default `config.yaml`.

3. Run the agent:
   ```sh
   vizier run --config .vizier/config.yaml
   ```

### Development Setup

For development, clone the repository and use the provided `just` commands:

```sh
# Install dependencies (Rust crates and webui npm packages)
just install

# Run in development mode with hot-reload
just dev

# Build the webui
just build
```

See the [Justfile](Justfile) for all available commands.

### Building without Python

To build Vizier without Python support (smaller binary, no Python dependency):

```sh
# Using cargo install
cargo install vizier --no-default-features

# Building from source
cargo build --release --no-default-features
```

**Benefits of building without Python:**
- No Python runtime required on user systems
- Smaller binary size
- Simplified distribution
- Python interpreter tool won't be available

**What remains available:**
- All other tools (CLI access, web search, scheduler, etc.)
- All channels (Discord, HTTP, WebUI)
- Full agent functionality

**Configuration Note:** If you build without Python, any agents with `python_interpreter` enabled will simply skip that tool without error.

### WebUI

The web interface is built with React and served automatically when the HTTP channel is enabled. After building (`just build`), it will be available at `http://localhost:9999` (or the port configured in your `config.yaml`).

## Update Installed Version

1. Install `cargo-update` if you haven't already:
   ```sh
   cargo install cargo-update
   ```

2. Update the binary:
   ```sh
   cargo install-update vizier
   ```

## Troubleshooting

### Python-Related Issues

#### Python Library Not Loaded Error

If you see an error like:
```
dyld[...]: Library not loaded: /Library/Frameworks/Python.framework/Versions/3.14/Python
```

This means the binary was built against a different Python version than what's on your system.

**Solutions:**

1. **Option 1: Build from source against your Python version**
   ```sh
   # Clone the repository
   git clone https://github.com/blinfoldking/vizier
   cd vizier

   # Build with your Python version
   PYO3_PYTHON=$(which python3.9) cargo install vizier
   ```

2. **Option 2: Build without Python support** (if you don't need Python interpreter)
   ```sh
   cargo install vizier --no-default-features
   ```

#### Wrong Python Version

If you see:
```
Python version X.X detected, but Python 3.9 or higher is required
```

Either:
- Install Python 3.9 or higher following the installation instructions above, OR
- Build without Python: `cargo install vizier --no-default-features`

#### Python Not Found

If you see:
```
Error: Could not find Python installation
```

This could mean:
- You built Vizier with Python support but Python isn't installed
- Python is installed but not in your PATH

**Solutions:**
- Install Python 3.9+, OR
- Build without Python: `cargo install vizier --no-default-features`


## Planned Features (V1.0.0)

- [x] Web UI (React-based interface)
- [x] Scheduler and task system (cron & one-time tasks)
- [x] Vector memory for long-term retention
- [x] Python interpreter tool
    - [x] Programmatic Tool Calling
- [x] Brave Search integration
- [x] Local embedding model support
- [ ] Docker Sandbox
- [ ] Model Context Protocol (MCP) integration
- [ ] Built-in HTTP client tool
- [ ] Skill system for reusable agent behaviors
- [ ] Additional AI providers (Google Gemini, OpenAI, Anthropic, etc.)
~~- [ ] TUI (terminal user interface)~~ on hold

## Development

### Project Structure

- `src/`: Rust source code (agents, channels, tools, scheduler, database, etc.)
- `webui/`: React-based web interface (built with Vite + React Router)
- `templates/`: Template files for agent configuration and identity
- `.vizier/`: Workspace directory for runtime data (config, database, agent workspaces)
- `migrations/`: Database migrations (SurrealDB schemas)

### Available Commands

See the [`Justfile`](Justfile) for available commands:

| Command | Description |
|---------|-------------|
| `just install` | Install all dependencies (Rust crates + webui npm packages) |
| `just dev` | Run in development mode with hot-reload |
| `just run` | Run in release mode |
| `just tui` | Start the terminal user interface (WIP) |
| `just docker` | Start Docker services (database, etc.) |
| `just build` | Build the webui frontend |

### CLI Commands

The `vizier` binary provides these subcommands:

- `vizier run --config <path>`: Start the agent with given config
- `vizier tui`: Launch the TUI client (requires running agent)
- `vizier init`: Initialize a new vizier workspace
- `vizier configure`: Generate a new config non-interactively

### Adding New Features

1. **New Tools**: Add to `src/agent/tools/` and register in `src/agent/tools/mod.rs`
2. **New Channels**: Add to `src/channels/` and implement the `Channel` trait
3. **New Models**: Extend the provider system in `src/agent/agent_impl/provider.rs`
4. **New Schedules**: Add to `src/scheduler/` and integrate with task database

## License

MIT License
