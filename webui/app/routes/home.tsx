import Editor from '~/components/editor'
import type { Route } from './+types/home'
import { useNavigate } from 'react-router'
import { createSession } from '~/services/vizier'

export function meta({ }: Route.MetaArgs) {
  return [
    { title: 'Vizier' },
    { name: 'description', content: '21st Century Digital Steward' },
  ]
}

export default function Home() {
  const naviagate = useNavigate()

  return (
    <>
      <div className="w-full h-full flex justify-center items-center bg-black text-white rounded-4xl">
        <div className="text-white">
          <div className="mb-5 font-bold">
            Hello, what's your plan today?
          </div>

          <Editor
            onSubmit={(value) => {
              createSession().then((res) =>
                naviagate(
                  `/chats/${res.data.data.session_id}`,
                  {
                    state: { prompt: value },
                  }
                )
              )
            }}
          />
        </div>
      </div>
    </>
  )
}
