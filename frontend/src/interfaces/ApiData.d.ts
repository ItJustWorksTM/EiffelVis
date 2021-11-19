interface IValues {
  meta: { id: string; type: string; version: string; time: number }
  data: {}
  links: Array<{ type: string; target: string }>
}

export default interface IData {
  events: IValues[]
}
