# 1.3 Configuration

Vizier uses a YAML-based configuration system with a main `.vizier.yaml` file and individual agent configuration files (`.agent.md` files).

## Configuration Structure

```
my-project/
├── .vizier.yaml              # Main configuration
├── .env                     # Environment variables (optional)
└── *.agent.md               # Agent definitions (e.g., vizier.agent.md)
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
    vector_memory:
      model: AllMiniLML6V2
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
- `openrouter` - OpenRouter.ai (supports many models)
- `deepseek` - DeepSeek AI
- `ollama` - Local Ollama instance

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
  vector_memory:
    model: AllMiniLML6V2                # Embedding model for memory
```

**Available Embedding Models:**
- `AllMiniLML6V2` (default) - Fast, good quality
- `AllMiniLML12V2` - Better quality, slower
- `BGELargeENV15` - Best quality
- `NomicEmbedTextV15` - Good balance
- `MxbaiEmbedLargeV1` - Large model
- And many more...

## Agent Configuration (`.agent.md`)

Agents are defined in Markdown files with YAML frontmatter. Create files like `vizier.agent.md`:

```markdown
---
name: "Vizier"
description: "Your personal AI assistant"
provider: openrouter
model: "anthropic/claude-3.5-sonnet"
session_ttl: "30m"
session_memory:
  max_capacity: 50
turn_depth: 10
tools:
  python_interpreter: true
  cli_access: false
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
---

You are Vizier, a helpful AI assistant. You serve as the right hand of your user.

[Rest of your system prompt here...]
```

### Agent Configuration Fields

| Field | Type | Description |
|-------|------|-------------|
| `name` | string | Display name for the agent |
| `description` | string | Brief description of the agent's purpose |
| `provider` | enum | AI provider: `openrouter`, `deepseek`, or `ollama` |
| `model` | string | Model identifier (provider-specific) |
| `session_ttl` | duration | Session timeout (e.g., `30m`, `1h`, `24h`) |
| `session_memory.max_capacity` | number | Max messages in short-term memory |
| `turn_depth` | number | How many messages to include in context |
| `tools` | object | Tool permissions (see below) |
| `silent_read_initiative_chance` | float | Probability (0-1) of agent initiating conversation |
| `show_thinking` | boolean | Whether to show agent's thinking process |

### Agent Tools Configuration

Each tool can be configured with:

```yaml
tools:
  python_interpreter: true              # Enable Python code execution
  cli_access: false                     # Enable shell command execution
  brave_search:
    enabled: true
    programmatic_tool_call: false       # Allow Python scripts to call this tool
  vector_memory:
    enabled: true
    programmatic_tool_call: true        # Allow Python to access memory
  discord:
    enabled: true                       # Enable Discord-specific actions
    programmatic_tool_call: false
```

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
    vector_memory:
      model: AllMiniLML6V2
```

**`assistant.agent.md`:**
```markdown
---
name: "Assistant"
description: "A helpful coding assistant"
provider: openrouter
model: "anthropic/claude-3.5-sonnet"
session_ttl: "1h"
session_memory:
  max_capacity: 100
turn_depth: 20
tools:
  python_interpreter: true
  cli_access: false
  brave_search:
    enabled: true
    programmatic_tool_call: true
  vector_memory:
    enabled: true
    programmatic_tool_call: false
show_thinking: true
---

You are a helpful coding assistant specialized in Rust and Python.
Help the user write clean, efficient code.
```

## Next Steps

- Learn about agents: [3.1 Overview](../agents/overview.md)
- Explore model providers: [3.5 Model Providers](../agents/providers.md)
- Understand the memory system: [3.4 Memory System](../agents/memory.md)
