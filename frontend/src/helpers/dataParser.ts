import IG6Data from '../interfaces/G6Data'
import IData from '../interfaces/ApiData'

export default (data: IData) => {
  const G6Data: IG6Data = { nodes: [], edges: [] }
  data.values.forEach(({ meta, links }) => {
    G6Data.nodes.push({ id: meta.id })
    links.forEach(({ target }) => {
      G6Data.edges.push({
        source: meta.id,
        target,
      })
    })
  })
  return G6Data
}
