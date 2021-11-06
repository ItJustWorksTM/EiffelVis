interface IValues {
  id: string
  edges: Array<string>
}

export default (data: IValues[]) => {
  const G6Data: any = { nodes: [], edges: [] }
  data.forEach(({ id, edges }) => {
    G6Data.nodes!.push({ id })
    console.log(id);
    edges.forEach((target: string) => {
      G6Data.edges!.push({
        source: id,
        target,
      })
    })
  })
  return G6Data
}
