import { useEffect, useRef, useState } from 'react'
import { OnConnect, OnMessage } from '../interfaces/types'

const useWebsocket = (onMsg: OnMessage, onConnect?: OnConnect) => {
  const [reconnecting, setReconnecting] = useState<boolean>(false)
  const socket = useRef<null | WebSocket>(null)

  useEffect(() => {
    socket.current = new WebSocket('ws://localhost:3001/ws')

    socket.current.onopen = () => {
      if (onConnect) onConnect()
    }

    socket.current.onmessage = (e) => onMsg(JSON.parse(e.data))

    socket.current.onclose = () => {
      if (socket.current) {
        if (reconnecting) return
        setReconnecting(true)
        setTimeout(() => setReconnecting(false), 2000)
      }
    }

    return () => {
      if (socket.current) socket.current.close()
      socket.current = null
    }
  }, [reconnecting])

  const sendMessage = (obj: object) => {
    if (socket.current) {
      if (socket.current!.readyState === 1) {
        socket.current.send(JSON.stringify(obj))
        return true
      }
    }
    return false
  }

  return {
    reconnecting,
    sendMessage,
  }
}

export default useWebsocket
