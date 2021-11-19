import IData from '../interfaces/ApiData'

export default (data: IData) => {

  const G6Data: any = { nodes: [], edges: [] }
  data.events.forEach(({ meta, links }) => {
    G6Data.nodes!.push({id: meta.id, time: meta.time})
    links.forEach(({ target }) => {
      G6Data.edges!.push({
        source: meta.id,
        target,
      })
    })
  })
  return G6Data
}
