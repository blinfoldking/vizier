import type { Chat } from '~/interfaces/chat'
import ReactMarkdown from 'react-markdown'
import DotLoader from './dot_loader'
import { motion, type Transition } from 'motion/react'

import remarkGfm from 'remark-gfm'

interface ChatBubbleProps {
  chat: Chat
}

const ChatBubble = (props: ChatBubbleProps) => {
  const isAgent = props.chat.user === 'agent'

  const username = (
    <div className="font-bold">
      {props.chat.user !== 'agent' ? `you` : 'vizier'}
    </div>
  )

  const Profile = () => {
    return (
      <div className="flex items-center">
        <div className="bg-black rounded-full w-10 h-10"></div>
      </div>
    )
  }

  const popUpAnimation = {
    initial: { scale: 0, opacity: 0 },
    animate: { scale: 1, opacity: 1 },
    // 'originX' and 'originY' can be used directly in the 'style' or as props
    style: { transformOrigin: !isAgent ? 'top right' : 'top left' },
    transition: { type: 'spring', stiffness: 260, damping: 20 },
  }

  return (
    <div
      className={`flex ${isAgent ? 'justify-start' : 'justify-end'} items-start w-full`}
    >
      <motion.div
        initial={popUpAnimation.initial}
        animate={popUpAnimation.animate}
        style={popUpAnimation.style}
        transition={popUpAnimation.transition as Transition<any>}
        className={`m-5 mt-0 mb-10 ${isAgent ? 'bg-gray-300 justify-start' : 'bg-white justify-end'} rounded-4xl p-5 flex items-start shadow-md w-fit overflow-hidden wrap-break-word`}
      >
        {isAgent && <Profile />}
        <div
          className={`m-5 mt-0 mb-0 flex flex-col ${!isAgent ? 'items-end' : 'items-start'}`}
        >
          {username}
          <div className="prose">
            {props.chat.content === 'thinking' ? (
              <div className="flex items-center">
                <div className="mr-4 font-bold">thinking</div>
                <DotLoader />
              </div>
            ) : (
              <ReactMarkdown remarkPlugins={[remarkGfm]}>
                {props.chat.content}
              </ReactMarkdown>
            )}
          </div>
          {props.chat.timestamp && (
            <div className="text-black opacity-50 text-xs mt-5">
              {props.chat.timestamp}
            </div>
          )}
        </div>
        {!isAgent && <Profile />}
      </motion.div>
    </div>
  )
}

export default ChatBubble
