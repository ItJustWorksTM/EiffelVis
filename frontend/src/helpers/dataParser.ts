import { GraphData } from '@antv/g6/lib/types'
import IData from '../interfaces/ApiData'

export default (data: IData) => {
  const G6Data: GraphData = { nodes: [], edges: [] }
  data.values.forEach(({ meta, links }) => {
    G6Data.nodes!.push({ id: meta.id })
    links.forEach(({ target }) => {
      G6Data.edges!.push({
        source: meta.id,
        target,
      })
    })
  })
  return G6Data
}
