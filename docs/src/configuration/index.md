# 2.1. Overview

This chapter covers everything you need to know about configuring Vizier.

## Overview

Vizier uses a two-file configuration system:

1. **`.vizier.yaml`** - Main configuration file containing providers, channels, tools, and global settings
2. **`.agent.md` files** - Individual agent definitions with YAML frontmatter

Both files support environment variable expansion using `${VAR}` syntax, allowing you to keep sensitive credentials in environment variables or `.env` files.

## Quick Reference

| File | Purpose | Location |
|------|---------|----------|
| `.vizier.yaml` | Main configuration (providers, channels, tools, storage, shell, embedding) | Project root |
| `*.agent.md` | Agent definitions (personality, model, memory, tools) | Project root |
| `.vizier/` | Workspace directory (auto-created) | Project root |

## Configuration Sections

- **[Main Configuration](./main-config.md)** - User identity and environment variables
- **[Providers](./providers.md)** - AI model provider settings
- **[Channels](./channels.md)** - Discord and HTTP server configuration
- **[Tools & Embedding](./tools-embedding.md)** - Global tool settings and embedding models
- **[Storage & Shell](./storage-shell.md)** - Data persistence and execution environment
- **[Agent Configuration](./agents.md)** - Complete reference for `.agent.md` files

## Environment Variables

All configuration files support environment variable expansion:

```yaml
providers:
  openrouter:
    api_key: "${OPENROUTER_API_KEY}"
```

Common environment variables:

| Variable | Used For |
|----------|----------|
| `OPENROUTER_API_KEY` | OpenRouter provider |
| `DEEPSEEK_API_KEY` | DeepSeek provider |
| `ANTHROPIC_API_KEY` | Anthropic provider |
| `OPENAI_API_KEY` | OpenAI provider |
| `GEMINI_API_KEY` | Gemini provider |
| `DISCORD_BOT_TOKEN` | Discord bot authentication |
| `BRAVE_API_KEY` | Brave Search API |

## Generating Configuration

### Initialize a New Project

```sh
vizier init
```

Creates:
- `.vizier.yaml` - Main configuration with sensible defaults
- `vizier.agent.md` - Sample agent definition
- `.vizier/` - Workspace directory for runtime data

## Loading Configuration

Vizier automatically looks for `.vizier.yaml` in the current directory. You can specify a custom path:

```sh
vizier run --config /path/to/.vizier.yaml
```

## Next Steps

- **[CLI Commands](./cli.md)** - `vizier init`, `configure`, and loading options
- Learn about [Agents](../configuration/agents.md)
