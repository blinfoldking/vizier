import { useEffect, useRef, useState } from 'react'
import { useLocation, useParams } from 'react-router'
import ChatBubble from '~/components/chat_bubble'
import Editor from '~/components/editor'
import type { Chat, WSChatResponse } from '~/interfaces/chat'
import { base_url } from '~/services/vizier'
import useWebSocket from 'react-use-websocket'
import { FaChevronDown } from 'react-icons/fa'
import { useSessionStore } from '~/hooks/sessionStore'

const Chat = () => {
  const [chats, setChats] = useState<Chat[]>([])
  const [isThinking, setIsThinking] = useState(false)
  const { sessionId } = useParams()
  const username = useSessionStore((state: any) => state.username)

  const sessionUrl = `ws://${base_url}/api/v1/session/${sessionId}/chat`
  const { sendJsonMessage, lastJsonMessage } = useWebSocket(sessionUrl)

  useEffect(() => {
    setChats([])
    setIsThinking(false)
  }, [sessionId])

  let bottomRef: any = useRef(null)
  const toBottom = () => {
    bottomRef.current?.scrollIntoView({ behaviour: 'smooth' })
  }

  useEffect(() => {
    if (!lastJsonMessage) return

    let res: WSChatResponse = lastJsonMessage as WSChatResponse

    setIsThinking(res.thinking)
    if (res.thinking) return

    let newChat: Chat = {
      user: 'agent',
      content: res.content,
      timestamp: new Date().toISOString(),
    }

    setChats([...chats, newChat])
  }, [lastJsonMessage])

  const location = useLocation()

  const send = (content: string) => {
    let newChat: Chat = {
      user: 'user',
      content: content,
      timestamp: new Date().toISOString(),
    }

    sendJsonMessage({
      user: username,
      content,
    })
    setChats([...chats, newChat])
    toBottom()
  }

  useEffect(() => {
    const initialPrompt = location.state?.prompt
    if (initialPrompt) send(initialPrompt)
  }, [])

  return (
    <div
      id="chatroom"
      className="h-full w-full flex flex-col justify-between relative overflow-hidden"
    >
      <div className="w-full overflow-y-scroll no-scrollbar p-5">
        {chats.map((chat, i) => (
          <ChatBubble key={i} chat={chat} />
        ))}
        {isThinking && (
          <ChatBubble chat={{ user: 'agent', content: 'thinking' }} />
        )}
        <div id="end" className="h-[25vh]" ref={bottomRef} />
      </div>
      <div className="absolute w-full flex flex-col justify-between bottom-0 p-5">
        <div></div>
        <div className="flex items-center h-full">
          <Editor onSubmit={send} />
          <div
            className="w-15 h-15 bg-white flex justify-center items-center ml-5 shadow-md hover:shadow-xl rounded-full"
            onClick={() => toBottom()}
          >
            <FaChevronDown />
          </div>
        </div>
      </div>
    </div>
  )
}

export default Chat
