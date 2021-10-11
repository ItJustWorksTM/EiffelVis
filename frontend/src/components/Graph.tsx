import React, { useEffect, useRef, useState } from 'react'
import axios from 'axios'
import G6, { Graph } from '@antv/g6'
import dataParser from '../helpers/dataParser'
import '../css/minimap.css'
import TooltipCard from './TooltipCard'

const link =
  'https://gist.githubusercontent.com/IdreesSamadi/6aa2e5f0f8c3828b41f1e3446d2002cd/raw/messages.json'

const CustomGraph: React.FC = () => {
  const [showNodeTooltip, setShowNodeTooltip] = useState<boolean>(false)
  const [nodeTooltipX, setNodeToolTipX] = useState<number>(0)
  const [nodeTooltipY, setNodeToolTipY] = useState<number>(0)
  const [nodeTooltipId, setNodeToolTipId] = useState<string>(' ')

  const graphContainer = useRef<any>(null)
  let graph: Graph | null = null

  const bindEvents = () => {
    if (graph) {
      graph.on('click', () => {
        setShowNodeTooltip(false)
      })
      graph.on('nodeselectchange', (e: any) => {
        if (e.select) {
          const config = e.target._cfg
          const {
            model: { id, x, y },
          } = config
          const point = graph!.getCanvasByPoint(x, y)
          setNodeToolTipX(point.x - 75)
          setNodeToolTipY(point.y + 15)
          setNodeToolTipId(id)
          setShowNodeTooltip(true)
        }
      })
    }
  }

  useEffect(() => {
    if (!graph) {
      const miniMap = new G6.Minimap({
        container: graphContainer.current,
        type: 'keyShape',
        className: 'g6MiniMap',
      })
      graph = new G6.Graph({
        container: graphContainer.current,
        width: window.innerWidth - 20,
        height: window.innerHeight - 10,
        fitView: true,
        modes: {
          default: [
            'drag-canvas',
            'drag-node',
            'click-select',
            {
              type: 'zoom-canvas',
              enableOptimize: true,
            },
            {
              type: 'activate-relations',
              trigger: 'click',
            },
          ],
        },
        layout: {
          type: 'dagre',
          workerEnabled: true,
          rankdir: 'LR',
          nodesep: 30,
          ranksep: 100,
        },
        plugins: [miniMap],
      })
    }

    axios
      .get(link)
      .then((response) => {
        const g6data = dataParser(response.data)
        graph!.data(g6data)
        graph!.render()
        bindEvents()
      })
      .catch((err) => err)
  }, [])
  // info: the reason behind not adding the window.screen.width as a dependency of useEffect is that we dont want to re-render the entire graph every time the window width changes

  return (
    <div ref={graphContainer}>
      {showNodeTooltip && (
        <TooltipCard id={nodeTooltipId} x={nodeTooltipX} y={nodeTooltipY} />
      )}
    </div>
  )
}

export default CustomGraph
