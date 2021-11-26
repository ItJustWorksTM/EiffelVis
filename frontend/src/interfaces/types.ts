/* eslint-disable no-unused-vars */
import Event, { ServerMessage } from './ApiData'

export type TweakCb = (obj: any) => void
export type OnMessage = (obj: string) => void
export type OnConnect = () => void
export type SendMessage = (obj: object) => void
export type getNodesWithRootFun = (id: string) => void
