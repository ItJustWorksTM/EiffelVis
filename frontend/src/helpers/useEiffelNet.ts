import { useEffect, useState } from 'react'
import { Collection, EventFilter, Forward, Query, RangeFilter, ServerMessage } from '../interfaces/ApiData'
import useWebsocket from './useWebSocket'

const useEiffelNet = (onEvents: any, onReset: any) => {
  const [awaitingResponse, setAwaitingResponse] = useState<Boolean>(false)
  const [awaitingResponseCount, setAwaitingResponseCount] = useState<number>(0)

  const [isConnected, setIsConnected] = useState<Boolean>(false)

  const [messageQueue, setMessageQueue] = useState<string[]>([])

  const onMessage = (event: string) => {
    setMessageQueue([...messageQueue, event])
  }

  useEffect(() => {
    messageQueue.forEach((eventBuf: string) => {
      const event = JSON.parse(eventBuf) as ServerMessage
      console.log('event ', event)
      if (Array.isArray(event)) {
        onEvents(event)
      } else {
        if (awaitingResponseCount === 0) {
          console.log('Unexpected mode change!')
        } else {
          setAwaitingResponseCount(awaitingResponseCount - 1)
        }

        if (event.error != null) {
          console.log('failed to switch modes! ', event)
        } else {
          console.log('ok')
          onReset()
        }
      }
    })
    if (messageQueue.length > 0) {
      setMessageQueue([])
    }
  }, [messageQueue])

  const onConnect = () => {
    setIsConnected(true)
  }

  const { reconnecting, sendMessage } = useWebsocket(onMessage, onConnect)
  useEffect(() => {
    if (reconnecting) {
      setIsConnected(false)
    }
  }, [reconnecting])

  const [rangeFilter, setRangeFilter] = useState<RangeFilter>({})
  const [filters, setFilters] = useState<EventFilter[]>([])
  const [collection, setCollection] = useState<Collection>(<Forward>{ type: 'Forward' })

  useEffect(() => {
    if (collection == null) {
      console.log('no collection specified, not querying')
    } else if (!isConnected) {
      console.log('web socket not connected')
    } else {
      const msg = <Query>{ range_filter: rangeFilter, event_filters: [filters], collection }
      console.log('sending out new query ', msg)
      if (sendMessage(JSON.stringify(msg))) {
        setAwaitingResponseCount(awaitingResponseCount + 1)
      } else {
        console.log('failed to send message, bad things may happen')
      }
    }
  }, [rangeFilter, filters, collection, isConnected])

  useEffect(() => {
    setAwaitingResponse(awaitingResponseCount > 0)
  }, [awaitingResponseCount])

  return {
    awaitingResponse,
    setRangeFilters: setRangeFilter,
    setFilters,
    setCollection,
  }
}

export default useEiffelNet
