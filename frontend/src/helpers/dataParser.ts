import chroma from "chroma-js"
import { Event } from '../interfaces/ApiData'


export default (data: Event[]) => {
  const G6Data: any = { nodes: [], edges: [] }
  // eslint-disable-next-line camelcase
  data.forEach(({ id, time, event_type, edges }) => {
    
    G6Data.nodes!.push({
      id,
      time,
      eventType: event_type,
      // the style should be base on something, not random
      color: chroma.hsl((edges.length * 100) % 365, 0.5, 0.5).hex(),
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
