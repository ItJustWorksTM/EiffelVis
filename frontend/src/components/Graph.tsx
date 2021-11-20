/* eslint-disable */
import React, { useEffect, useRef, useState } from 'react'
import { Graph } from '@antv/g6'
import dataParser from '../helpers/dataParser'
import '../css/minimap.css'
import TooltipCard from './TooltipCard'
import { GraphData } from '@antv/g6/lib/types'
import styles from '../css/graph.module.css'
import Loader from './Loader'
import useTweakPane from '../helpers/useTweakPane'
import G6 from '../helpers/timeLayout'
import useWebsocket from '../helpers/useWebSocket'

const CustomGraph: React.FC = () => {
  const [showNodeTooltip, setShowNodeTooltip] = useState<boolean>(false)
  const [nodeTooltipX, setNodeToolTipX] = useState<number>(0)
  const [nodeTooltipY, setNodeToolTipY] = useState<number>(0)
  const [nodeTooltipId, setNodeToolTipId] = useState<string>(' ')
  const graphContainer = useRef<any>(null)
  const graphRef = useRef<Graph | null>(null)

  const bindEvents = () => {
    const graph = graphRef.current
    if (graph) {
      graph.data({})
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

  const onMessage = (event: any) => {
    let graph = graphRef.current

    console.log('from server', event)
    if (Array.isArray(event)) {
      const g6data: GraphData = dataParser(event)
      if (graph) {
        g6data.nodes!.forEach((node: any) => {
          graph!.addItem('node', node)
        })
        if (g6data.edges) {
          g6data.edges.forEach((edge: any) => {
            graph!.addItem('edge', edge)
          })
        }
        graph?.layout()
        console.log(
          'TOTAL NODES: ',
          (graph!.save() as GraphData)!.nodes!.length
        )
      }
    } else {
      console.log('Type I DONT KNOW', event)
      graph!.data({})
      graph!.render()
    }
  }

  const { reconnecting, sendMessage } = useWebsocket(onMessage)
  const getNodesWithThisRoot = (id: string) => {
    // socket.current.send(JSON.stringify({ type: 'WithRoot', ids: [id] }))
  }

  useEffect(() => {
    // connect()

    if (!graphRef.current) {
      const miniMap = new G6.Minimap({
        container: graphContainer.current,
        type: 'keyShape',
        className: 'g6MiniMap',
      })
      graphRef.current = new G6.Graph({
        container: graphContainer.current,
        width: window.innerWidth - 73,
        height: window.innerHeight - 10,
        fitView: true,
        defaultEdge: {
          style: {
            endArrow: { path: G6.Arrow.triangle(10, 20, 0), d: 0 },
          },
        },
        modes: {
          default: [
            'drag-node',
            'click-select',
            'drag-canvas',
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
          type: 'timeLayout',
        },
        plugins: [miniMap],
      })
    }

    // socket.current.onmessage = onMessage
    bindEvents()
    console.log('use 01')
    console.log('graoh', graphRef.current)
    return () => {
      // socket.current.close()
    }
  }, [])
  // info: the reason behind not adding the window.screen.width as a dependency of useEffect is that we dont want to re-render the entire graph every time the window width changes

  useTweakPane(({ id, type }: any) => {
    const ret: any = {
      filters: [{ type: 'Type', name: 'Unique' }],
      collection: { type: 'AsRoots' },
    }
    // ret.filters.push({ type: "None" })
    sendMessage(ret)
  })
  const loader = reconnecting && <Loader />
  return (
    <div>
      {loader}
      <div className={styles.graphContainer} ref={graphContainer}>
        {showNodeTooltip && (
          <TooltipCard
            id={nodeTooltipId}
            x={nodeTooltipX}
            y={nodeTooltipY}
            getNodesWithRoot={getNodesWithThisRoot}
          />
        )}
      </div>
    </div>
  )
}

export default CustomGraph
