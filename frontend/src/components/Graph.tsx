/* eslint-disable */
import React, { useEffect, useRef, useState } from 'react'
import G6, { Graph } from '@antv/g6'
import dataParser from '../helpers/dataParser'
import '../css/minimap.css'
import TooltipCard from './TooltipCard'
import { GraphData } from '@antv/g6/lib/types'
import { Pane } from 'tweakpane'
import styles from '../css/graph.module.css'
import Loader from './Loader'

const CustomGraph: React.FC = () => {
  const [showNodeTooltip, setShowNodeTooltip] = useState<boolean>(false)
  const [nodeTooltipX, setNodeToolTipX] = useState<number>(0)
  const [nodeTooltipY, setNodeToolTipY] = useState<number>(0)
  const [nodeTooltipId, setNodeToolTipId] = useState<string>(' ')
  const [isLoading, setIsLoading] = useState<boolean>(false)
  const [requestData, setRequestData] = useState<any>({
    type: 'All',
    id: ''
  })

  const graphContainer = useRef<any>(null)
  let graph: Graph | null = null
  let pane: Pane


  const bindEvents = () => {
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

  useEffect(() => {
    const socket = new WebSocket('ws://localhost:3001/ws')

    if (!graph) {
      const miniMap = new G6.Minimap({
        container: graphContainer.current,
        type: 'keyShape',
        className: 'g6MiniMap',
      })
      graph = new G6.Graph({
        container: graphContainer.current,
        width: window.innerWidth - 73,
        height: window.innerHeight - 10,
        fitView: true,
        defaultEdge: {
    style:{
        endArrow: {path: G6.Arrow.triangle(10, 20, 100),d: 0}
    }
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
          type: 'dagre',
          workerEnabled: false,
          rankdir: 'LR',
          nodesep: 30,
          ranksep: 100,
        },
        plugins: [miniMap],
      })
    }
    pane = new Pane({ title: 'Events', expanded: true })
    pane.addInput(requestData, 'type', {
      label: 'Type',
      options: { All: 'All', WithRoot: 'WithRoot' },
    })
    pane.addSeparator()
    pane.addInput(requestData, 'id', {
      label: 'node id',
      step: 1,
    })

    pane
      .addButton({
        title: 'New Nodes',
      })
      .on('click', () => {
        setIsLoading(true)
        socket.send(JSON.stringify(pane.exportPreset()))
        console.log('preset', requestData)
      })
    pane.on('change', () => {
      console.log('changed', pane.exportPreset())
      setRequestData(pane.exportPreset())
    })

    socket.addEventListener('message', (event) => {
      console.log('from server', event.data)
      let le = JSON.parse(event.data)
      console.log('from server', le)
      if (Array.isArray(le)) {
        const g6data: any = dataParser(le)
        setTimeout(() => {
          setIsLoading(false)
        }, 700)
        console.log('else', isLoading)
        g6data.nodes.forEach((node: any) => {
          graph!.addItem('node', {
            ...node,
            x: Math.random() * 1000,
            y: Math.random() * 1000,
          })
        })
        g6data.edges.forEach((edge: any) => {
          graph!.addItem('edge', edge)
        })

        console.log("TOTAL NODES: ", (graph!.save() as GraphData)!.nodes!.length)

        bindEvents()
      } else if (le['type'] == 'All') {
        setIsLoading(false)
        console.log('Type ALL', le)
        graph!.data({})
        graph!.render()
      } else if (le['type'] == 'WithRoot') {
        setIsLoading(false)
        console.log('Type WithRoot', le)
        graph!.data({})
        graph!.render()
      }
    })
  }, [])
  // info: the reason behind not adding the window.screen.width as a dependency of useEffect is that we dont want to re-render the entire graph every time the window width changes
  const loader = isLoading && <Loader />
  return (
    <div>
      {loader}
      <div className={styles.graphContainer} ref={graphContainer}>
        {showNodeTooltip && (
          <TooltipCard id={nodeTooltipId} x={nodeTooltipX} y={nodeTooltipY} />
        )}
      </div>
    </div>
  )
}

export default CustomGraph
