'use client'

import { FaCog } from 'react-icons/fa'
import { TbPlus } from 'react-icons/tb'
import { Link, Outlet, useLocation } from 'react-router'
import {
  ping,
  base_url,
  listSession as listSessions,
  createSession,
} from './services/vizier'
import { useEffect, useState } from 'react'
import { useSessionStore } from './hooks/sessionStore'

const OnboardModal = () => {
  const [username, setUsername] = useState('')
  const storeUsername = useSessionStore((state: any) => state.setUsername)

  return (
    <>
      <div className="bg-transparent h-screen w-screen absolute top-0 left-0 z-100 backdrop-blur-sm"></div>
      <div className="bg-black h-screen w-screen absolute top-0 left-0 z-101 opacity-75"></div>
      <div className=" h-screen w-screen absolute top-0 left-0 z-101 flex justify-center items-center">
        <div className="bg-white w-75 p-5 rounded-4xl shadow-2xl flex justify-center items-center flex-col">
          <div className="font-bold text-2xl mb-5">Welcome!</div>
          <div>
            <input
              placeholder="Enter your name"
              className="inset-shadow-md p-2 pl-5 pr-5 rounded-full bg-white"
              onChange={(ev) => setUsername(ev.target.value)}
            ></input>
          </div>
          <button
            className="mt-5 active:inset-shadow-md p-5 pt-2 pb-2 rounded-full hover:font-bold"
            onClick={() => storeUsername(username.trim())}
          >
            Enter
          </button>
        </div>
      </div>
    </>
  )
}

const Layout = () => {
  const [connected, setConnected] = useState(false)
  const [sessions, setSessions] = useState([])
  const location = useLocation()

  const username = useSessionStore((state: any) => state.username)

  const updateSessions = () => {
    listSessions().then((res) => setSessions(res.data.data))
  }

  useEffect(() => {
    updateSessions()
  }, [connected, location])

  const checkStatus = () => {
    ping()
      .then((res) => setConnected(res?.data?.status === 200))
      .catch(() => setConnected(false))
  }

  useEffect(() => {
    checkStatus()
    setInterval(() => checkStatus(), 5000)
  }, [])

  return (
    <>
      {!username && <OnboardModal />}
      <div className="bg-white flex justify-between w-screen h-screen pr-0">
        <div
          id="sidebar"
          className="w-75 pt-12 pb-12 flex flex-col justify-between"
        >
          <div>
            <div
              className="p-5 w-full flex  border-b-2 border-dashed border-b-gray-500"
              id="profile"
            >
              <div className="w-10 h-10 bg-black mr-2.5 rounded-xl"></div>
              <div>
                <div className="flex items-center">
                  <div>{username ?? 'placeholder'}</div>
                  <div
                    className={`ml-2.5 w-2.5 h-2.5 ${connected ? 'bg-emerald-500' : 'bg-red-500'} rounded-full`}
                  ></div>
                </div>
                <div className="text-xs text-gray-400">
                  http://{base_url}
                </div>
              </div>
            </div>
            <div className="p-5 pb-0">
              <div className="flex items-center justify-between font-bold">
                <Link to={'/'}>/index</Link>
              </div>
            </div>

            <div className="p-5 pb-0">
              <div className="flex items-center justify-between">
                <strong>/chats</strong>
                <div
                  className="flex"
                  onClick={() => {
                    createSession().then(() =>
                      updateSessions()
                    )
                  }}
                >
                  <TbPlus />
                </div>
              </div>
              {sessions.map((session) => (
                <div className="pl-4">
                  <Link to={`/chats/${session}`}>
                    /{session}
                  </Link>
                </div>
              ))}
            </div>

            <div className="p-5 pb-0">
              <div>
                <strong>/tools</strong>
              </div>
              <div className="pl-4">
                <div>/memory</div>
              </div>
              <div className="pl-4">
                <div>/document</div>
              </div>
            </div>
          </div>
          <div className="p-5 pt-2.5 pb-2.5 text-gray-500 bg-white font-bold flex items-center active:inset-shadow-md hover:text-black rounded-full m-5 mb-0">
            <FaCog size={20} />
            <button className="ml-2 select-none">Settings</button>
          </div>
        </div>
        <div className="w-full p-5 pl-0 pr-0">
          <div
            id="content"
            className="w-full h-full rounded-l-4xl p-1 inset-shadow-md overflow-hidden"
            style={{ background: '#ddd' }}
          >
            {/*TODO*/}
            {username ? (
              <Outlet />
            ) : (
              <div className="w-full h-full bg-black"></div>
            )}
          </div>
        </div>
      </div>
    </>
  )
}

export default Layout
