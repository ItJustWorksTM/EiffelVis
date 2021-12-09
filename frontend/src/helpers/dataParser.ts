import { Event } from '../interfaces/ApiData'
import { nodeColor } from './useLayout'

export default (data: Event[]) => {
  const G6Data: any = { nodes: [], edges: [] }
  // eslint-disable-next-line camelcase
  data.forEach(({ id, time, event_type, edges }) => {
    G6Data.nodes!.push({
      id,
      date: String(time),
      time,
      eventType: event_type,
      size: 10,
      // the style should be base on something, not random
      style: {
        fill: nodeColor(event_type),
        lineWidth: 0.4,
      },
    })
    edges.forEach((target: string) => {
      G6Data.edges!.push({
        source: id,
        target,
      })
    })
  })
  return G6Data
}
