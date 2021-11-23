/* eslint-disable no-unused-vars */
import IEvent from './ApiData'

export type TweakCb = (obj: object) => void
export type OnMessage = (obj: IEvent[]) => void
export type OnConnect = () => void
export type SendMessage = (obj: object) => void
export type getNodesWithRootFun = (id: string) => void
