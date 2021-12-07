/* eslint-disable no-unused-vars */

export type TweakCb = (obj: any) => void
export type OnMessage = (obj: string) => void
export type OnConnect = () => void
export type SendMessage = (obj: object) => void
export type getNodesWithRootFun = (id: string) => void
export type getDate = (date: number) => void
export type _getTime = (date: string) => void
export type onSubmitDrawer = (data: IDrawerSubmitData) => void
export interface ICalender {
  day: number
  month: number
  year: number
}

export interface IDrawerSubmitData {
  collection: string
  filters: string
  beginTime: string
  endTime: string
  nodeId: string
  typeName: string
}
