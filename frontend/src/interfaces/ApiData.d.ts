// Interally tagged type for types that are part of a enum
type TypeTag<K, T> = K & { type: T }

export type Uuid = string

export interface Event {
  id: Uuid
  time: number
  edges: Array<Uuid>
}

interface _Time {
  begin?: number
  end?: number
}
export type Time = TypeTag<_Time, 'Time'>

interface _Type {
  name: string
}
export type Type = TypeTag<_Type, 'Type'>

interface _Ids {
  ids: Array<Uuid>
}
export type Ids = TypeTag<_Ids, 'Ids'>

export type Forward = TypeTag<_Forward, 'Forward'>
interface _Forward {}

export type AsRoots = TypeTag<_AsRoots, 'AsRoots'>
interface _AsRoots {}

export type Filter = null | Time | Type | Ids
export type Collection = null | Forward | AsRoots

export interface Query {
  filters: Filter[]
  collection: Collection
}

export interface QueryRes {
  repl: string
  error?: string
}

export type ServerMessage = QueryRes | Event[]
