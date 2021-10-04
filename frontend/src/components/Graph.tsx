import React, { useEffect, useState } from 'react'
import axios from 'axios'
import Graphin, { Behaviors } from '@antv/graphin'
import { MiniMap, Tooltip } from '@antv/graphin-components'
import dataParser from '../helpers/dataParser'
import G6Data from '../interfaces/G6Data'
import CustomTooltip from './CustomTooltip'

const { ZoomCanvas, ActivateRelations, Hoverable, ClickSelect } = Behaviors

const link =
  'https://gist.githubusercontent.com/IdreesSamadi/6aa2e5f0f8c3828b41f1e3446d2002cd/raw/messages.json'

const Graph: React.FC = () => {
  const [data, setData] = useState<G6Data>()
  const [mapStyle, setMapStyle] = useState<object>({
    background: '#000',
    bottom: '3rem',
    left: 'auto',
    right: '0',
  })
  useEffect(() => {
    if (window.screen.width < 1024) {
      setMapStyle({
        background: '#000',
        left: 'auto',
        right: '0',
        top: '1rem',
        bottom: 'unset',
      })
    }
    axios
      .get(link)
      .then((response) => {
        const g6data = dataParser(response.data)
        setData(g6data)
      })
      .catch((err) => err)
  }, [])
  // info: the reason behind not adding the window.screen.width as a dependency of useEffect is that we dont want to re-render the entire graph every time the window width changes

  return data ? (
    <Graphin
      data={data}
      theme={{ mode: 'dark' }}
      layout={{ type: 'fruchterman', rankdir: 'LR', gpuEnabled: true }}
    >
      <Tooltip bindType="node" placement="right">
        <CustomTooltip />
      </Tooltip>
      <ActivateRelations trigger="click" />
      <ClickSelect />
      <Hoverable />
      <MiniMap visible style={mapStyle} />
      <ZoomCanvas />
    </Graphin>
  ) : (
    <div>loading...</div>
  )
}

export default Graph
