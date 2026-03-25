# 2.5 Tools & Embedding

## `tools`

Global tool settings:

```yaml
tools:
  dangerously_enable_cli_access: false  # Allow shell command execution
  brave_search:
    api_key: "${BRAVE_SEARCH_API_KEY}"
    safesearch: true                    # Enable safe search
```

### Tool Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `dangerously_enable_cli_access` | bool | `false` | Enable shell command execution globally |
| `brave_search.api_key` | string | `"${BRAVE_API_KEY}"` | Brave Search API key |
| `brave_search.safesearch` | bool | `true` | Enable safe search filtering |

## `embedding`

Configure embedding models for vector memory:

```yaml
embedding:
  type: local
  model: all_mini_lml6_v2
```

### Local Models

Set `type: local` and choose from 31+ local models (via fastembed):

| Model | Size | Use Case |
|-------|------|----------|
| `all_mini_lml6_v2` | ~22MB | Fast, good quality (default) |
| `all_mini_lml12_v2` | ~33MB | Better quality, slower |
| `bge_large_env15` | ~1.3GB | Best quality |
| `nomic_embed_text_v15` | ~540MB | Good balance |
| `mxbai_embed_large_v1` | ~1.3GB | Large model |
| `multilingual_e5_large` | ~1.3GB | Multilingual support |

### Cloud Providers

```yaml
embedding:
  type: openrouter
  model: "openai/text-embedding-3-small"
```

Supported cloud providers: `openrouter`, `ollama`, `openai`, `gemini`
