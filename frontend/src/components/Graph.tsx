import React, { useEffect, useRef, useState } from 'react'
import { Graph, GraphData, TimeBar } from '@antv/g6'
import G6 from '../helpers/useCustomShapes'
import dataParser from '../helpers/dataParser'
import '../css/minimap.css'
import '../css/timebar.css'
import TooltipCard from './TooltipCard'
import { layout, resetLayout } from '../helpers/useLayout'
import styles from '../css/graph.module.css'
import Loader from './Loader'
import useTweakPane from '../helpers/useTweakPane'
import {
  AsRoots,
  Collection,
  Event,
  EventFilter,
  Forward,
  TimeBarData,
} from '../interfaces/ApiData'
import useEiffelNet from '../helpers/useEiffelNet'

const CustomGraph: React.FC = () => {
  const [showNodeTooltip, setShowNodeTooltip] = useState<boolean>(false)
  const [nodeTooltipX, setNodeToolTipX] = useState<number>(0)
  const [nodeTooltipY, setNodeToolTipY] = useState<number>(0)
  const [nodeTooltipTime, setNodeToolTipTime] = useState<number>(0)
  const [nodeTooltipType, setNodeToolTipType] = useState<string>(' ')
  const [nodeTooltipId, setNodeToolTipId] = useState<string>(' ')
  const [timeBarData, setTimeBarData] = useState<TimeBarData[]>([])
  const [offset, setOffset] = useState<number>(0)
  const totalTrendCountRef = useRef<number>(0)
  const timeBarRef = useRef<any>(null)
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

  const timeBar = () => {
    const graph = graphRef.current
    if (graph) {
      if (timeBarData.length > 0) {
        if (timeBarRef.current) {
          graph.removePlugin(timeBarRef.current)
          console.log('TimeBar removed')
        }
        timeBarRef.current = new TimeBar({
          className: 'g6TimeBar',
          x: 0,
          y: 0,
          width: 900,
          height: 110,
          padding: 10,
          type: 'trend',
          trend: {
            data: timeBarData,
            smooth: true,
          },
          slider: {
            backgroundStyle: {
              fill: '#000000',
            },
            foregroundStyle: {
              fill: '#626262',
            },
            handlerStyle: {
              style: {
                fill: '#ad0c04',
                stroke: '#ad0c04',
              },
            },
            textStyle: {
              fill: '#ffffff',
            },
          },
          controllerCfg: {
            fill: '#000000',
            stroke: '#000000',
            timePointControllerText: ' Point',
            timeRangeControllerText: ' Point',
          },
          /* TimeBarSliceOption: {
            tickLabelFormatter: (d: any) => {

            } 
          } */
        })
        graph!.addPlugin(timeBarRef.current)
        console.log('TimeBar added')
        console.log('TIMEBAR DATA: ', timeBarData)
      }
    }
  }

  const onMessage = (event: Event[]) => {
    const graph = graphRef.current
    const timeBarDataCache: TimeBarData[] = timeBarData
    if (graph) {
      const g6data: GraphData = dataParser(event)
      graph!.setAutoPaint(false)
      let timeStamp: number = 0
      let timeStampCount: number = 0
      g6data.nodes!.forEach((node: any, i: number) => {
        layout(node, offset)
        graph!.addItem('node', node)
        if (node.y === 0) {
          graph!.focusItem(node.id, false, {
            easing: 'easeCubic',
            duration: 1000,
          })
        }
        // When indexing before last index of message
        if (timeStamp === 0) {
          if (
            timeBarDataCache.length !== 0 &&
            node.time ===
              Number(timeBarDataCache[timeBarDataCache.length - 1].date)
          ) {
            const timeBarDataCacheEnd = timeBarDataCache.pop()
            timeStamp = Number(timeBarDataCacheEnd!.date)
            timeStampCount = Number(timeBarDataCacheEnd!.value)
            totalTrendCountRef.current -= timeStampCount
          } else {
            timeStamp = node.time
          }
        }
        if (node.time !== timeStamp) {
          timeBarDataCache.push({
            date: String(timeStamp),
            value: String(timeStampCount),
          })
          totalTrendCountRef.current += timeStampCount
          timeStamp = node.time
          timeStampCount = 0
        }
        timeStampCount += 1
        // When indexing last index of message
        if (g6data.nodes!.length - 1 === i) {
          timeBarDataCache.push({
            date: String(timeStamp),
            value: String(timeStampCount),
          })
          totalTrendCountRef.current += timeStampCount
        }
      })

      if (g6data.edges) {
        g6data.edges.forEach((edge) => {
          graph!.addItem('edge', edge)
        })
      }
      graph!.setAutoPaint(true)

      setTimeBarData(timeBarDataCache)
      graph.data(graph!.save() as GraphData)
      if (timeBarRef) {
        timeBar()
      }
      console.log('TREND RECORDED EVENTS: ', totalTrendCountRef.current)
      console.log('TOTAL NODES: ', (graph!.save() as GraphData)!.nodes!.length)
    }
  }

  const onReset = () => {
    const graph = graphRef.current
    graph?.data({})
    setTimeBarData([])
    totalTrendCountRef.current = 0
    graph?.render()
    resetLayout()
    setShowNodeTooltip(false)
  }

  const { awaitingResponse, setFilters, setCollection } = useEiffelNet(
    onMessage,
    onReset
  )

  const getNodesWithThisRoot = (id: string) => {
    console.log(id)
    setFilters([{ rev: false, pred: { type: 'Id', ids: [id] } }])
    setCollection({ type: 'AsRoots' } as AsRoots)
    setShowNodeTooltip(false)
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
        autoPaint: false,
        defaultEdge: {
          type: 'custom',
          style: {
            endArrow: { path: G6.Arrow.triangle(5, 10, 0), d: 0 },
          },
        },
        nodeStateStyles: {
          selected: {
            fill: '#ffffff',
            lineWidth: 0.4,
          },
        },
        modes: {
          default: [
            {
              type: 'drag-canvas',
              enableOptimize: true,
            },
            {
              type: 'zoom-canvas',
              enableOptimize: true,
              optimizeZoom: 0.9,
            },
            'click-select',
          ],
        },

        layout: {},
        plugins: [miniMap],
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

    setOffset(obj.offset)
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
