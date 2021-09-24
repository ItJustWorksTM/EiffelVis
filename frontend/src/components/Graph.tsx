import React, { useEffect, useState } from 'react'
import axios from 'axios'
import Graphin, { Behaviors } from '@antv/graphin'
import dataParser from '../helpers/dataParser'
import G6Data from '../interfaces/G6Data'

const { ZoomCanvas } = Behaviors

const link =
  'https://gist.githubusercontent.com/IdreesSamadi/6aa2e5f0f8c3828b41f1e3446d2002cd/raw/messages.json'

const Graph: React.FC = () => {
  const [data, setData] = useState<G6Data>()

  useEffect(() => {
    axios
      .get(link)
      .then((response) => {
        const g6data = dataParser(response.data)
        setData(g6data)
      })
      .catch((err) => console.log(err))
  }, [])

  return data ? (
    <Graphin
      data={data}
      fitView
      fitCenter
      theme={{ mode: 'dark' }}
      layout={{ type: 'circular' }}
    >
      <ZoomCanvas />
    </Graphin>
  ) : (
    <div>loading...</div>
  )
}

export default Graph