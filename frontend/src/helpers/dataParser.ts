interface IValues {
  id: string
  time: number
  edges: Array<string>
}

export default (data: IValues[]) => {
  const G6Data: any = { nodes: [], edges: [] }
  data.forEach(({ id, time, edges }) => {
    G6Data.nodes!.push({ id, time })
    // console.log(id)
    edges.forEach((target: string) => {
      G6Data.edges!.push({
        source: id,
        target,
      })
    })
  })
  return G6Data
}
