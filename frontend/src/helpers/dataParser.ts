import { Event } from '../interfaces/ApiData'

export default (data: Event[]) => {
  const G6Data: any = { nodes: [], edges: [] }
  // eslint-disable-next-line camelcase
  data.forEach(({ id, time, event_type, edges }) => {
    G6Data.nodes!.push({
      id,
      date: String(time),
      time,
      eventType: event_type,
      size: 1,
      // the style should be base on something, not random
      style: {
        fill: `hsl(${(edges.length * 100) % 365}, 50%, 50%)`,
        lineWidth: 0.1,
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
