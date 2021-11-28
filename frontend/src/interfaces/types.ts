/* eslint-disable no-unused-vars */
import Event, { ServerMessage } from './ApiData'

export type TweakCb = (obj: any) => void
export type OnMessage = (obj: ServerMessage) => void
export type OnConnect = () => void
export type SendMessage = (obj: object) => void
export type getNodesWithRootFun = (id: string) => void
export type sendData = (collection: string, filter: string, begin: number, end: number, id:string) => void