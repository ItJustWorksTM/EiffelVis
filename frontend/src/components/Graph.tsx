import React, { useEffect, useRef, useState } from 'react'
import G6, { Graph } from '@antv/g6'
import { GraphData } from '@antv/g6/lib/types'
import dataParser from '../helpers/dataParser'
import '../css/minimap.css'
import TooltipCard from './TooltipCard'
import styles from '../css/graph.module.css'
import Loader from './Loader'
// import useTweakPane from '../helpers/useTweakPane'
import { AsRoots, Collection, Event, Filter, Forward, Ids } from '../interfaces/ApiData'
import useEiffelNet from '../helpers/useEiffelNet'
import Menu from './Menu'

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

  const onMessage = (event: Event[]) => {
    const graph = graphRef.current

    const g6data: GraphData = dataParser(event)
    if (graph) {
      g6data.nodes!.forEach((node) => {
        graph!.addItem('node', node)
      })
      if (g6data.edges) {
        g6data.edges.forEach((edge) => {
          graph!.addItem('edge', edge)
        })
      }
      graph?.layout()
      console.log(
        'TOTAL NODES: ',
        (graph!.save() as GraphData)!.nodes!.length
      )
    }
  }

  const onReset = () => {
    const graph = graphRef.current
    graph!.data({})
    graph!.render()
  }
  

  const { awaitingResponse, setFilters, setCollection, } = useEiffelNet(onMessage, onReset)

  const getNodesWithThisRoot = (id: string) => {
    setFilters([{ type: "Ids", ids: [id] } as Ids])
    setCollection({ type: "AsRoots" } as AsRoots)
  }
  useEffect(() => {
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
          type: 'dagre',
        },
        plugins: [miniMap],
      })
    }

    bindEvents()

    setCollection({ type: "Forward"} as Forward)
  }, [])
  // info: the reason behind not adding the window.screen.width as a dependency of useEffect is that we dont want to re-render the entire graph every time the window width changes

  // useTweakPane((obj) => {
  //   const collection = { type: obj.collection_type } as Collection
  //   const filter = { type: obj.filter_type, ...obj, ids: [obj.id] } as Filter

  //   setCollection(collection)
  //   setFilters([filter])
  // })
  const handleFilterSubmit = (collection: string, filter: string, begin: number, end: number, ids:string) => {
    const obj = {
      collection_type:collection, 
      filter_type: filter,
      time: begin,
      id: ids,
      // nodetype: type 
    }; 
     const thiscollection = { type: obj.collection_type } as Collection
     const thisfilter = { type: obj.filter_type, ...obj, ids: [obj.id] } as Filter

     setCollection(thiscollection)
     setFilters([thisfilter])
     }
     
  const loader = awaitingResponse && <Loader />
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
     <Menu handleSubmit={handleFilterSubmit}/>
    </div>
  )
}

export default CustomGraph
