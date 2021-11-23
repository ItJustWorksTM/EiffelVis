import { GraphData } from '@antv/g6/lib/types'
import IEvent from '../interfaces/ApiData'

export default (data: IEvent[]) => {
  const G6Data: GraphData = { nodes: [], edges: [] }
  data.forEach(({ id, time, edges }) => {
    G6Data.nodes!.push({
      id,
      time,
      // the style should be base on something, not random
      style: { fill: `hsl(${(edges.length * 100) % 365}, 50%, 50%)` },
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
