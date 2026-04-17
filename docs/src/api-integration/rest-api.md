# 3.1 REST API

Vizier exposes a REST API with Swagger documentation at `http://localhost:9999/swagger`.

## Authentication

Vizier uses JWT-based authentication.

### Obtaining a Token

**Endpoint:** `POST /api/v1/auth/login`

Send your credentials to receive a JWT token:

```json
{
  "username": "your_username",
  "password": "your_password"
}
```

**Response:**

```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

### Using the Token

Once you have a token, include it in subsequent requests:

- **REST endpoints:** Add `Authorization: Bearer <token>` header
- **WebSocket:** Append `?token=<token>` query parameter to the URL

## WebSocket Chat

The WebSocket endpoint provides real-time chat interaction with agents.

### Endpoint

```
/api/v1/agents/{agent_id}/channel/{channel_id}/topic/{topic_id}/chat
```

**Example full URL:**
```
ws://localhost:9999/api/v1/agents/agent1/channel/1/topic/1/chat?token=your_jwt_token
```

### Message Format (Client → Server)

```json
{
  "timestamp": "2025-04-18T12:00:00Z",
  "user": "username",
  "content": { "chat": "Hello, how are you?" },
  "metadata": {}
}
```

**Content types:**

| Type | Description |
|------|-------------|
| `chat` | Regular chat message |
| `prompt` | Prompt request, same as chat but with no persistance history |
| `silent_read` | Silent read operation |
| `command` | Command to execute |

### Response Format (Server → Client)

```json
{
  "timestamp": "2025-04-18T12:00:01Z",
  "content": {
    "thinking_start": null,
    "thinking": "Let me think about this...",
    "tool_choice": { "name": "http_client", "args": {} },
    "message": { "content": "Response text", "stats": {...} }
  }
}
```

**Response content types:**

| Type | Description |
|------|-------------|
| `thinking_start` | Agent started thinking |
| `thinking` | Reasoning output |
| `tool_choice` | Tool being executed (includes `name` and `args`) |
| `message` | Final response with `content` and optional `stats` |
| `abort` | Response was aborted |

## Other Endpoints

For complete API documentation including all available endpoints, visit:

```
http://localhost:9999/swagger
```

The Swagger UI provides interactive documentation where you can:

- Explore all available endpoints
- Test API calls directly from the browser
- View request/response schemas
- Download the OpenAPI specification
