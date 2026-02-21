# Vizier - Multi-Channel AI Assistant Framework

Vizier is a Rust-based AI agent framework that provides a unified interface for AI assistants across multiple communication channels (Discord, HTTP, etc.) with memory, tool usage, and extensible architecture.

## Features

- **Multi-Channel Support**: Connect to Discord, HTTP, and other communication platforms
- **AI Model Integration**: Support for multiple AI providers (DeepSeek, OpenRouter, Ollama, etc.)
- **Memory System**: Session-based memory with configurable recall depth
- **Tool System**: Extensible tool framework with CLI access, web search, and vector memory
- **TUI Interface**: Built-in terminal user interface for local interaction
- **Docker Support**: Easy deployment with Docker and Docker Compose
- **Configuration Driven**: Flexible configuration via TOML files

## Architecture

Vizier follows a modular architecture:

```
src/
├── agent/           # Core agent logic and session management
├── channels/        # Communication channel implementations
├── config.rs        # Configuration parsing and management
├── transport.rs     # Message transport layer
├── utils/          # Utility functions
└── vizier/         # Main application logic and CLI
```

## Configuration

to setup your initial config, run this command:
```sh
vizier onboard
```

## Development

### Project Structure

- `src/`: Rust source code
- `templates/`: Template files for agent configuration
- `migrations/`: Database migrations (if using SQL)
- `data/`: Persistent data storage
- `.vizier/`: Workspace directory for runtime data

### Available Commands

See the `Justfile` for available commands:
- `just dev`: Run in development mode
- `just run`: Run in release mode  
- `just tui`: Start TUI interface
- `just docker`: Start Docker services

### Adding New Features

1. **New Tools**: Add to `src/agent/tools/`
2. **New Channels**: Add to `src/channels/`
3. **New Models**: Extend the model provider system in `src/agent/mod.rs`

## License

MIT License
