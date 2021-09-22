export interface INodes {
  id: string
  x?: number
  y?: number
  label?: string
  labelCfg?: any
  style?: any
}

export interface IEdges {
  target: string
  source: string
  label?: string
  style?: any
}

export default interface IData {
  nodes: INodes[]
  edges: IEdges[]
}
