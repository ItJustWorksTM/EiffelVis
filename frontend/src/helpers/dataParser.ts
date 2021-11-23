import { Event } from '../interfaces/ApiData'

export default (data: Event[]) => {
  const G6Data: any = { nodes: [], edges: [] }
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
