/* eslint-disable no-unused-vars */

export type TweakCb = (obj: any) => void
export type OnMessage = (obj: string) => void
export type OnConnect = () => void
export type SendMessage = (obj: object) => void
export type getNodesWithRootFun = (id: string) => void
export type getDate = (date: number) => void
export type _getTime = (date: string) => void
export interface ICalender {
  day: number
  month: number
  year: number
}
