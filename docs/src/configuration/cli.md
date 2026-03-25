# 2.8 CLI Commands

## Generating Configuration

### Initialize a New Project

```sh
vizier init
```

Creates:
- `.vizier.yaml` - Main configuration
- `vizier.agent.md` - Sample agent definition
- `.vizier/` directory - Workspace for database and files

## Loading Configuration

Vizier automatically looks for `.vizier.yaml` in the current directory. You can specify a custom path:

```sh
vizier run --config /path/to/.vizier.yaml
```

## Configuration Loading Order

1. Load `.vizier.yaml` from current directory (or specified path)
2. Scan for all `*.agent.md` files in the same directory
3. Parse agent configurations from frontmatter
4. Create `.vizier/` workspace directory for runtime data

## Environment Variable Expansion

Vizier supports environment variable expansion in configuration files using the `${VAR}` syntax:

```yaml
providers:
  openrouter:
    api_key: "${OPENROUTER_API_KEY}"
```

This allows you to keep sensitive credentials in environment variables or `.env` files while keeping your configuration clean. The following fields support environment variable expansion:

- All API keys in `providers.*.api_key`
- Discord tokens in `channels.discord.*.token`
- Brave Search API key in `tools.brave_search.api_key`
- Any other string field in the configuration

### Example `.env` file

```bash
OPENROUTER_API_KEY=sk-or-v1-...
DISCORD_TOKEN=MTA0...
BRAVE_API_KEY=BS...
```
