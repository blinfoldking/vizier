# 1.3 Configuration

Vizier uses a YAML-based configuration system with a main `.vizier.yaml` file and individual agent configuration files (`.agent.md` files).

## Configuration Structure

```
my-project/
├── .vizier.yaml              # Main configuration
├── .env                      # Environment variables (optional)
└── *.agent.md                # Agent definitions (e.g., vizier.agent.md)
```

## Main Configuration File (`.vizier.yaml`)

The main configuration is a single `.vizier.yaml` file at your project root:

```yaml
vizier:
  primary_user:
    name: "Your Name"
    discord_id: "123456789"
    discord_username: "yourusername"
    alias: ["you", "master", "sir"]

  providers:
    openrouter:
      api_key: "${OPENROUTER_API_KEY}"
    deepseek:
      api_key: "${DEEPSEEK_API_KEY}"
    ollama:
      base_url: "http://localhost:11434"

  channels:
    discord:
      vizier:
        token: "${DISCORD_TOKEN}"
    http:
      port: 9999

  tools:
    dangerously_enable_cli_access: false
    brave_search:
      api_key: "${BRAVE_SEARCH_API_KEY}"
      safesearch: true

  embedding:
    type: local
    model: all_mini_lml6_v2

  storage:
    type: filesystem

  shell:
    environment: local
    path: "."
```

## Configuration Sections

### `primary_user`

Defines the primary user who interacts with agents:

```yaml
primary_user:
  name: "Your Name"                    # Your name
  discord_id: "123456789"              # Your Discord user ID (optional)
  discord_username: "username"         # Your Discord username (optional)
  alias: ["you", "master", "boss"]     # Aliases the agent can use for you
```

### `providers`

Configure AI model providers. At least one provider must be configured:

```yaml
providers:
  openrouter:
    api_key: "your-api-key"
  
  deepseek:
    api_key: "your-api-key"
  
  ollama:
    base_url: "http://localhost:11434"  # Default Ollama URL
```

**Supported Providers:**

| Provider | Config | Description |
|----------|--------|-------------|
| `openrouter` | `api_key` | [OpenRouter.ai](https://openrouter.ai) - Access 200+ models with a single API key |
| `deepseek` | `api_key` | [DeepSeek](https://deepseek.com) - High-quality Chinese and English models |
| `ollama` | `base_url` | Local Ollama instance (default: `http://localhost:11434`) |
| `anthropic` | `api_key` | [Anthropic Claude](https://anthropic.com) models |
| `openai` | `api_key`, `base_url` | OpenAI models (custom base_url for compatibility with OpenAI-compatible APIs) |
| `gemini` | `api_key` | [Google Gemini](https://ai.google.dev) models |

Example configuration with all providers:

```yaml
providers:
  openrouter:
    api_key: "${OPENROUTER_API_KEY}"
  
  deepseek:
    api_key: "${DEEPSEEK_API_KEY}"
  
  ollama:
    base_url: "http://localhost:11434"
  
  anthropic:
    api_key: "${ANTHROPIC_API_KEY}"
  
  openai:
    api_key: "${OPENAI_API_KEY}"
    base_url: null  # Optional: for OpenAI-compatible APIs
  
  gemini:
    api_key: "${GEMINI_API_KEY}"
```

### `channels`

Configure communication channels:

```yaml
channels:
  discord:                              # Discord bot configuration
    vizier:                             # Agent-specific Discord config
      token: "${DISCORD_TOKEN}"
    assistant:                          # Another agent's Discord config
      token: "${DISCORD_TOKEN_2}"
  
  http:                                 # HTTP/WebSocket server
    port: 9999                          # Default port
```

### `tools`

Global tool settings:

```yaml
tools:
  dangerously_enable_cli_access: false  # Allow shell command execution
  brave_search:
    api_key: "${BRAVE_SEARCH_API_KEY}"
    safesearch: true                    # Enable safe search
```

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `dangerously_enable_cli_access` | bool | `false` | Enable shell command execution globally |
| `brave_search.api_key` | string | `"${BRAVE_API_KEY}"` | Brave Search API key |
| `brave_search.safesearch` | bool | `true` | Enable safe search filtering |

### `embedding`

Configure embedding models for vector memory:

```yaml
embedding:
  type: local
  model: all_mini_lml6_v2
```

**Local Models:**

Set `type: local` and choose from 31+ local models (via fastembed):

| Model | Size | Use Case |
|-------|------|----------|
| `all_mini_lml6_v2` | ~22MB | Fast, good quality (default) |
| `all_mini_lml12_v2` | ~33MB | Better quality, slower |
| `bge_large_env15` | ~1.3GB | Best quality |
| `nomic_embed_text_v15` | ~540MB | Good balance |
| `mxbai_embed_large_v1` | ~1.3GB | Large model |
| `multilingual_e5_large` | ~1.3GB | Multilingual support |

**Cloud Providers:**

```yaml
embedding:
  type: openrouter
  model: "openai/text-embedding-3-small"
```

Supported cloud providers: `openrouter`, `ollama`, `openai`, `gemini`

### `storage`

Configure data persistence backend:

```yaml
storage:
  type: filesystem
```

| Type | Description |
|------|-------------|
| `filesystem` | Store data in `.vizier/` directory (default) |
| `surreal` | Use SurrealDB for data storage |

### `shell`

Configure the execution environment for shell commands:

```yaml
shell:
  environment: local
  path: "."
```

**Local Environment:**

```yaml
shell:
  environment: local
  path: "/path/to/working/dir"  # Working directory for shell commands
```

**Docker Environment:**

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

## Agent Configuration (`.agent.md`)

Agents are defined in Markdown files with YAML frontmatter. Create files like `vizier.agent.md`:

```markdown
---
name: "Vizier"
description: "Your personal AI assistant"
provider: openrouter
model: "anthropic/claude-3.5-sonnet"
session_memory:
  max_capacity: 50
thinking_depth: 10
prompt_timeout: "5m"
session_timeout: "30m"
tools:
  timeout: "1m"
  python_interpreter: true
  shell_access: false
  brave_search:
    enabled: true
    programmatic_tool_call: false
  vector_memory:
    enabled: true
    programmatic_tool_call: true
  discord:
    enabled: true
    programmatic_tool_call: false
silent_read_initiative_chance: 0.1
show_thinking: true
include_documents:
  - "docs/**/*.md"
---

You are Vizier, a helpful AI assistant. You serve as the right hand of your user.

[Rest of your system prompt here...]
```

### Agent Configuration Fields

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | string | required | Display name for the agent |
| `description` | string | `null` | Brief description of the agent's purpose |
| `provider` | enum | required | AI provider: `openrouter`, `deepseek`, `ollama`, `anthropic`, `openai`, `gemini` |
| `model` | string | required | Model identifier (provider-specific) |
| `session_memory.max_capacity` | number | required | Max messages in short-term memory |
| `thinking_depth` | number | required | How many previous messages to include in context |
| `prompt_timeout` | duration | `"5m"` | Tool execution timeout |
| `session_timeout` | duration | `"30m"` | Session TTL before automatic cleanup |
| `silent_read_initiative_chance` | float | `0.0` | Probability (0-1) of agent initiating conversation |
| `show_thinking` | boolean | `null` | Whether to show agent's thinking process |
| `include_documents` | array | `null` | Glob patterns for additional context files |

### Agent Tools Configuration

Each tool can be configured with:

```yaml
tools:
  timeout: "1m"                         # Global tool execution timeout
  python_interpreter: false             # Enable Python code execution
  shell_access: false                   # Enable shell command execution
  brave_search:
    enabled: false
    programmatic_tool_call: false       # Allow Python scripts to call this tool
  vector_memory:
    enabled: true
    programmatic_tool_call: true        # Allow Python to access memory
  discord:
    enabled: false                      # Enable Discord-specific actions
    programmatic_tool_call: false
```

**Tool Options:**

| Tool | `enabled` | `programmatic_tool_call` | Note |
|------|-----------|-------------------------|------|
| `python_interpreter` | N/A (use `true`/`false`) | N/A | Requires `--features python` build |
| `shell_access` | N/A (use `true`/`false`) | N/A | Subject to global `dangerously_enable_cli_access` |
| `brave_search` | Enable web search | Allow Python to invoke search | Requires `BRAVE_API_KEY` |
| `vector_memory` | Enable memory | Allow Python to use memory | Requires embedding config |
| `discord` | Enable Discord actions | Allow Python to use Discord | Requires Discord token in `.vizier.yaml` |

**Note:** `python_interpreter` requires building with `--features python`.

## Generating Configuration

### Initialize a New Project

```sh
vizier init
```

Creates:
- `.vizier.yaml` - Main configuration
- `vizier.agent.md` - Sample agent definition
- `.vizier/` directory - Workspace for database and files

### Non-Interactive Configuration

```sh
vizier configure
```

Generates a minimal `.vizier.yaml` without prompts.

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

## Complete Example

**`.vizier.yaml`:**
```yaml
vizier:
  primary_user:
    name: "Alice"
    discord_id: "123456789"
    discord_username: "alice_user"
    alias: ["Alice", "Boss"]

  providers:
    openrouter:
      api_key: "${OPENROUTER_API_KEY}"
    ollama:
      base_url: "http://localhost:11434"

  channels:
    http:
      port: 9999
    discord:
      assistant:
        token: "${DISCORD_TOKEN}"

  tools:
    dangerously_enable_cli_access: false
    brave_search:
      api_key: "${BRAVE_API_KEY}"
      safesearch: true

  embedding:
    type: local
    model: all_mini_lml6_v2

  storage:
    type: filesystem

  shell:
    environment: local
    path: "."
```

**`assistant.agent.md`:**
```markdown
---
name: "Assistant"
description: "A helpful coding assistant"
provider: openrouter
model: "anthropic/claude-3.5-sonnet"
session_memory:
  max_capacity: 100
thinking_depth: 20
prompt_timeout: "5m"
session_timeout: "1h"
tools:
  timeout: "1m"
  python_interpreter: true
  shell_access: false
  brave_search:
    enabled: true
    programmatic_tool_call: true
  vector_memory:
    enabled: true
    programmatic_tool_call: false
  discord:
    enabled: true
    programmatic_tool_call: false
show_thinking: true
include_documents:
  - "docs/**/*.md"
---

You are a helpful coding assistant specialized in Rust and Python.
Help the user write clean, efficient code.
```

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

**Example `.env` file:**

```bash
OPENROUTER_API_KEY=sk-or-v1-...
DISCORD_TOKEN=MTA0...
BRAVE_API_KEY=BS...
```

## Next Steps

- Return to [Getting Started](../getting-started/quick-start.md)
- Explore [Development](../development/overview.md) documentation
