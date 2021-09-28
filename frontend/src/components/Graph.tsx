import React, { useEffect, useState } from 'react'
import axios from 'axios'
import Graphin, { Behaviors, GraphinContext } from '@antv/graphin'
import { MiniMap, Tooltip } from '@antv/graphin-components'
import Card from './Card'
import dataParser from '../helpers/dataParser'
import G6Data from '../interfaces/G6Data'

const { ZoomCanvas, ActivateRelations, Hoverable, ClickSelect } = Behaviors

const link =
  'https://gist.githubusercontent.com/IdreesSamadi/6aa2e5f0f8c3828b41f1e3446d2002cd/raw/messages.json'

const CustomTooltip = () => {
  const { tooltip } = React.useContext(GraphinContext)
  const context = tooltip.node
  const { item } = context
  const model = item && item.getModel()

  return (
    <div>
      <Card id={model.id} />
    </div>
  )
}

const Graph: React.FC = () => {
  const [data, setData] = useState<G6Data>()

  useEffect(() => {
    axios
      .get(link)
      .then((response) => {
        const g6data = dataParser(response.data)
        setData(g6data)
      })
      .catch((err) => err)
  }, [])

  return data ? (
    <Graphin
      data={data}
      fitView
      fitCenter
      theme={{ mode: 'dark' }}
      layout={{ type: 'circular' }}
    >
      <Tooltip bindType="node" placement="right">
        <CustomTooltip />
      </Tooltip>
      <ActivateRelations trigger="click" />
      <ClickSelect />
      <Hoverable />
      <MiniMap
        visible
        style={{
          background: '#000000',
          marginBottom: '3rem',
          marginLeft: '0rem',
        }}
      />
      <ZoomCanvas />
    </Graphin>
  ) : (
    <div>loading...</div>
  )
}

export default Graph
