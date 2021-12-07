import React, { useEffect, useRef, useState } from 'react'
import G6, { Graph, GraphData } from '@antv/g6'
import dataParser from '../helpers/dataParser'
import '../css/minimap.css'
import TooltipCard from './TooltipCard'
import styles from '../css/graph.module.css'
import Loader from './Loader'
import useTweakPane from '../helpers/useTweakPane'
import {
  AsRoots,
  Collection,
  Event,
  EventFilter,
  Forward,
} from '../interfaces/ApiData'
import useEiffelNet from '../helpers/useEiffelNet'

let timee = 0
let posx = 0
let posy = 0
let log = 1

const CustomGraph: React.FC = () => {
  const [showNodeTooltip, setShowNodeTooltip] = useState<boolean>(false)
  const [nodeTooltipX, setNodeToolTipX] = useState<number>(0)
  const [nodeTooltipY, setNodeToolTipY] = useState<number>(0)
  const [nodeTooltipTime, setNodeToolTipTime] = useState<number>(0)
  const [nodeTooltipType, setNodeToolTipType] = useState<string>(' ')
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
            model: { id, time, eventType, x, y },
          } = config
          const point = graph!.getCanvasByPoint(x, y)
          setNodeToolTipX(point.x - 75)
          setNodeToolTipY(point.y + 15)
          setNodeToolTipTime(time)
          setNodeToolTipType(eventType)
          setNodeToolTipId(id)
          setShowNodeTooltip(true)
        }
      })
    }
  }

  const layout = (node: any) => {
    const temp = node
    const tempTime: number = temp.time
    if (tempTime <= timee + 1000) {
      temp.x = posx
      if (posy < 0) {
        temp.y = posy
        posy = posy * -1 + 100 * 0.99 ** log
        log += 1
      } else {
        temp.y = posy
        if (posy !== 0) {
          posy *= -1
        }
      }
    } else if (tempTime > timee) {
      posx += 100
      temp.x = posx
      posy = 0
      log = 1
      temp.y = posy
      posy += 100
      timee = tempTime
    }
  }

  const onMessage = (event: Event[]) => {
    const graph = graphRef.current
    if (graph) {
      const g6data: GraphData = dataParser(event)
      graph!.setAutoPaint(false)    
      const track = ''
      g6data.nodes!.forEach((node: any) => {
        layout(node)
        graph!.addItem('node', node)
      })
      if (track !== '') {
        graph!.focusItem(track, true, {
          easing: 'easeCubic',
          duration: 400,
        })
      }
      if (g6data.edges) {
        g6data.edges.forEach((edge) => {
          graph!.addItem('edge', edge)
        })
      }
      graph!.setAutoPaint(true)    

    }
  }

  const onReset = () => {
    const graph = graphRef.current
    graph!.data({})
    graph!.render()
    timee = 0
    posx = 0
    posy = 0
    log = 1
    setShowNodeTooltip(false)
  }

  const { awaitingResponse, setFilters, setCollection } = useEiffelNet(
    onMessage,
    onReset
  )

  const getNodesWithThisRoot = (id: string) => {
    console.log(id)
    setFilters([{ rev: false, pred: { type: 'Id', id }}])
    setCollection({ type: 'AsRoots' } as AsRoots)
    setShowNodeTooltip(false)
  }

  useEffect(() => {
    if (!graphRef.current) {

      graphRef.current = new G6.Graph({
        container: graphContainer.current,
        width: window.innerWidth - 73,
        height: window.innerHeight - 10,
        fitView: true,
        maxZoom: 20,
        defaultEdge: {
          style: {
            endArrow: { path: G6.Arrow.triangle(10, 20, 0), d: 0 },
          },
        },
        modes: {
          default: [
            'click-select',
            'drag-canvas',
            {
              type: 'zoom-canvas',
              optimizeZoom: 0.9,
              enableOptimize: true,
            },
          ],
        },

        layout: {},
        plugins: [],
      })
    }

    bindEvents()

    setCollection({ type: 'Forward' } as Forward)
  }, [])
  // info: the reason behind not adding the window.screen.width as a dependency of useEffect is that we dont want to re-render the entire graph every time the window width changes

  useTweakPane((obj) => {
    const collection = { type: obj.collection_type } as Collection
    const filter = {
      type: obj.filter_type,
      ...obj,
      ids: [obj.id],
      begin: obj.begin >= 0 ? obj.begin : null,
      end: obj.end >= 0 ? obj.end : null,
    } as EventFilter

    setCollection(collection)
    setFilters([filter])
  })

  const loader = awaitingResponse && <Loader />
  return (
    <div>
      {loader}
      <div className={styles.graphContainer} ref={graphContainer}>
        {showNodeTooltip && (
          <TooltipCard
            id={nodeTooltipId}
            time={nodeTooltipTime}
            eventType={nodeTooltipType}
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
