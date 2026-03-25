# 2.4 Channels

## `channels`

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

## Discord Channel

Each agent can have its own Discord bot configuration:

```yaml
channels:
  discord:
    <agent_name>:
      token: "${DISCORD_TOKEN}"
```

## HTTP Channel

Configure the HTTP/WebSocket server:

```yaml
channels:
  http:
    port: 9999
```
