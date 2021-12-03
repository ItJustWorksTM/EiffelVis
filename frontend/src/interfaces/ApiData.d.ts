/* eslint-disable camelcase */
// Interally tagged type for types that are part of a enum
type TypeTag<K, T> = K & { type: T }

export type Uuid = string

export interface Event {
  id: Uuid
  time: number
  // eslint-disable-next-line camelcase
  event_type: string
  edges: Array<Uuid>
}

interface _Absolute {
  val: number
}
export type Absolute = TypeTag<_Absolute, 'Absolute'>

interface _Time {
  val: number
}
export type Time = TypeTag<_Time, 'Time'>

interface _Ids {
  val: Uuid
}
export type Ids = TypeTag<_Ids, 'Ids'>

interface _Type {
  name: string
}
export type Type = TypeTag<_Type, 'Type'>

interface _SourceHost {
  host: string,
}
export type SourceHost = TypeTag<_SourceHost, 'SourceHost'>

interface _SourceName {
  name: string
}
export type SourceName = TypeTag<_SourceName, 'SourceName'>

interface _Tag {
  tag: string
}
export type Tag = TypeTag<_Tag, 'Tag'>

interface _Id {
  id: Uuid
}
export type Id = TypeTag<_Id, 'Id'>

export type Forward = TypeTag<_Forward, 'Forward'>
interface _Forward {}

export type AsRoots = TypeTag<_AsRoots, 'AsRoots'>
interface _AsRoots {}


export type EventFilterType = Type | Id | SourceHost | SourceName | Tag
export interface EventFilter {
  rev: boolean,
  pred: EventFilterType
}
export type Collection = Forward | AsRoots
export type RangeFilterBound = Absolute | Time | Ids

export interface RangeFilter {
  begin?: RangeFilterBound,
  end?: RangeFilterBound
}

export interface Query {
  range_filter?: RangeFilter,
  event_filters: EventFilter[][],
  collection: Collection
}

export interface QueryRes {
  repl: string
  error?: string
}

export type ServerMessage = QueryRes | Event[]

export interface TimeBarData {
  date: string
  value: string
}

