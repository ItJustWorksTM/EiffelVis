interface IValues {
  id: string
  time: number
  edges: Array<string>
}

export default (data: IValues[]) => {
  const G6Data: any = { nodes: [], edges: [] }
  data.forEach(({ id, time, edges }) => {
    // const col = edges.length > 0 ? 255 : 0
    G6Data.nodes!.push({ id, time, style: { fill: `hsl(${edges.length * 100 % 365}, 50%, 50%)`} })
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
