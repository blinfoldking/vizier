export interface Chat {
  user: 'agent' | 'user'
  content: 'thinking' | string
  timestamp?: string
}

export interface WSChatResponse {
  content: string
  thinking: boolean
}
