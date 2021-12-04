import React, { useEffect, useRef, useState } from 'react'
import '../css/minimap.css'
import {
  useLoadGraph,
  useRegisterEvents,
  useSigma,
} from 'react-sigma-v2/lib/esm'
import Sigma from 'sigma'
import dataParser from '../helpers/dataParser'
import useTweakPane from '../helpers/useTweakPane'
// import styles from '../css/graph.module.css'
import {
  AsRoots,
  // AsRoots,
  Collection,
  Event,
  Filter,
  Forward,
  Ids,
  // Ids,
} from '../interfaces/ApiData'
import TooltipCard from './TooltipCard'
import useEiffelNet from '../helpers/useEiffelNet'

let timee = 0
let posx = 0
let posy = 0
let log = 1

const CustomGraph: React.FC = () => {
  const [showNodeTooltip, setShowNodeTooltip] = useState<boolean>(false)

  const [nodeTooltipTime, setNodeToolTipTime] = useState<number>(0)
  const [nodeTooltipType, setNodeToolTipType] = useState<string>(' ')
  const [nodeTooltipId, setNodeToolTipId] = useState<string>(' ')
  const sigma = useRef<Sigma>(useSigma())
  const events = useRegisterEvents()

  const loadGraph = useLoadGraph()

  const bindEvents = () => {
    const graph = sigma.current.getGraph()
    console.log(sigma.current.getSettings())
    events({
      clickNode: (ev) => {
        const { time, eventType } = graph.getNodeAttributes(ev.node)
        // setNodeToolTipX(x - 75)
        // setNodeToolTipY(y + 15)
        setNodeToolTipTime(time)
        setNodeToolTipType(eventType)
        setNodeToolTipId(ev.node)
        setShowNodeTooltip(true)
      },
    })
    if (graph) {
      // graph.data({})
      // graph.on('click', () => {
      //   setShowNodeTooltip(false)
      // })
      // graph.on('nodeselectchange', (e: any) => {
      //   if (e.select) {
      //     const config = e.target._cfg
      //     const {
      //       model: { id, time, eventType, x, y },
      //     } = config
      //     const point = graph!.getCanvasByPoint(x, y)
      //     setNodeToolTipX(point.x - 75)
      //     setNodeToolTipY(point.y + 15)
      //     setNodeToolTipTime(time)
      //     setNodeToolTipType(eventType)
      //     setNodeToolTipId(id)
      //     setShowNodeTooltip(true)
      //   }
      // })
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
    const graph = sigma.current.getGraph()
    const g6data = dataParser(event)
    if (graph) {
      g6data.nodes!.forEach((node: any) => {
        if (!graph.hasNode(node.id)) {
          layout(node)
          graph.addNode(node.id, {
            size: 2,
            x: node.x,
            y: node.y,
            color: node.color,
          })
        }
      })

      if (g6data.edges) {
        g6data.edges.forEach((edge: any) => {
          if (graph.hasNode(edge.source) && graph.hasNode(edge.target)) {
            if (!graph.hasEdge(edge.source, edge.target)) {
              graph.addEdge(edge.source, edge.target, {
                size: 0.5,
                color: 'rgba(255,255,255)',
                type: 'arrow',
              })
            }
          }
        })
      }
      console.log('TOTAL NODES: ', graph.nodes.length)
    }
  }

  const onReset = () => {
    const graph = sigma.current.getGraph()
    graph.clear()
    // graph!.data({})
    // graph!.render()
    timee = 0
    posx = 0
    posy = 0
    log = 1
    // setShowNodeTooltip(false)
  }

  const { setFilters, setCollection } = useEiffelNet(onMessage, onReset)

  const getNodesWithThisRoot = (id: string) => {
    setFilters([{ type: 'Ids', ids: [id] } as Ids])
    setCollection({ type: 'AsRoots' } as AsRoots)
    setShowNodeTooltip(false)
  }

  useEffect(() => {
    const graph = sigma.current.getGraph()

    graph.addNode('Jessica', {
      label: 'Jessica',
      x: 1,
      y: 1,
      color: '#FF0',
      size: 10,
    })
    graph.addNode('Truman', {
      label: 'Truman',
      x: 0,
      y: 0,
      color: '#00F',
      size: 5,
    })
    graph.addEdge('Jessica', 'Truman', { color: '#CCC', size: 1 })

    loadGraph(graph)
    // const miniMap = new G6.Minimap({
    //   container: graphContainer.current,
    //   type: 'keyShape',
    //   className: 'g6MiniMap',
    // })
    // graphRef.current = new G6.Graph({
    //   container: graphContainer.current,
    //   width: window.innerWidth - 73,
    //   height: window.innerHeight - 10,
    //   fitView: true,
    //   defaultEdge: {
    //     style: {
    //       endArrow: { path: G6.Arrow.triangle(10, 20, 0), d: 0 },
    //     },
    //   },
    //   modes: {
    //     default: [
    //       'click-select',
    //       'drag-canvas',
    //       {
    //         type: 'zoom-canvas',
    //         enableOptimize: true,
    //       },
    //     ],
    //   },

    //   layout: {},
    //   plugins: [miniMap],
    // })

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
    } as Filter

    setCollection(collection)
    setFilters([filter])
  })

  return (
    <div>
      <div style={{ position: 'static', left: 0, top: 0 }}>
        {showNodeTooltip && (
          <TooltipCard
            id={nodeTooltipId}
            time={nodeTooltipTime}
            eventType={nodeTooltipType}
            x={60}
            y={0}
            getNodesWithRoot={getNodesWithThisRoot}
          />
        )}
      </div>
    </div>
  )
}

export default CustomGraph
