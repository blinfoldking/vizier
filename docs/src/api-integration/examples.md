# 3.2 Examples

## JavaScript

### Authentication

```javascript
const response = await fetch('http://localhost:9999/api/v1/auth/login', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({ username: 'user', password: 'pass' })
});
const { token } = await response.json();
```

### WebSocket Chat

```javascript
const ws = new WebSocket(
  'ws://localhost:9999/api/v1/agents/agent1/channel/1/topic/1/chat?token=' + token
);

ws.onopen = () => {
  console.log('Connected to chat');
  ws.send(JSON.stringify({
    timestamp: new Date().toISOString(),
    user: 'username',
    content: { chat: 'Hello, how are you?' },
    metadata: {}
  }));
};

ws.onmessage = (event) => {
  const response = JSON.parse(event.data);
  if (response.content.thinking) {
    console.log('Thinking:', response.content.thinking);
  }
  if (response.content.message) {
    console.log('Message:', response.content.message.content);
  }
  if (response.content.tool_choice) {
    console.log('Tool:', response.content.tool_choice.name);
  }
};

ws.onerror = (error) => console.error('WebSocket error:', error);
ws.onclose = () => console.log('Disconnected');
```

## Python

### Authentication

```python
import requests

response = requests.post(
    'http://localhost:9999/api/v1/auth/login',
    json={'username': 'user', 'password': 'pass'}
)
token = response.json()['token']
```

### WebSocket Chat

```python
import websocket
import json
from datetime import datetime

ws = websocket.create_connection(
    'ws://localhost:9999/api/v1/agents/agent1/channel/1/topic/1/chat?token=' + token
)

message = {
    'timestamp': datetime.utcnow().isoformat(),
    'user': 'username',
    'content': {'chat': 'Hello, how are you?'},
    'metadata': {}
}
ws.send(json.dumps(message))

while True:
    response = json.loads(ws.recv())
    content = response.get('content', {})
    if content.get('thinking'):
        print('Thinking:', content['thinking'])
    if content.get('message'):
        print('Message:', content['message']['content'])
    if content.get('tool_choice'):
        print('Tool:', content['tool_choice']['name'])
```