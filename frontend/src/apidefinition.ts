/* eslint-disable camelcase */
// Interally tagged type for types that are part of a enum
type TypeTag<T, K> = K & { type: T }

export type Uuid = string

export interface FullEvent {
  meta: { id: Uuid, type: string, version: string, time: number, tags?: string[], source?: { host: string, name: string } }
  data: {}
  links: { type: string, target: Uuid }[]
}

export interface Event {
  id: Uuid
  time: number
  event_type: string
  edges: Array<Uuid>
}

export type Type = TypeTag<'Type', {
  names: string[]
}>

export type SourceHost = TypeTag<'SourceHost', {
  hosts: string[]
}>

export type SourceName = TypeTag<'SourceName', {
  names: string[]
}>

export type Tag = TypeTag<'Tag', {
  tags: string[]
}>

export type Id = TypeTag<'Id', {
  ids: Uuid[]
}>

export type EventFilterType = Type | Id | SourceHost | SourceName | Tag
export const event_filter_type_eq = (lhs: EventFilterType, rhs: EventFilterType) =>
  lhs.type == rhs.type &&
  ((): boolean => {
    const _rhs = rhs as any; // stupid linter...
    switch (lhs.type) {
      case "Id": return lhs.ids.every((val) => _rhs.ids.includes(val))
      case "SourceHost": return lhs.hosts.every((val) => _rhs.hosts.includes(val))
      case "SourceName": return lhs.names.every((val) => _rhs.names.includes(val))
      case "Tag": return lhs.tags.every((val) => _rhs.tags.includes(val))
      case "Type": return lhs.names.every((val) => _rhs.names.includes(val))
    }
  })()

export interface EventFilter<T> {
  rev: boolean
  pred: T
}

export const event_filter_eq = (lhs: EventFilter<EventFilterType>, rhs: EventFilter<EventFilterType>) =>
  lhs.rev == rhs.rev && event_filter_type_eq(lhs.pred, rhs.pred)

export type Forward = TypeTag<'Forward', {}>
export type AsRoots = TypeTag<'AsRoots', {}>
export type Collection = Forward | AsRoots

export const collection_eq = (lhs: Collection, rhs: Collection): boolean => lhs.type == rhs.type

export type Absolute = TypeTag<'Absolute', {
  val: number
}>

export type Time = TypeTag<'Time', {
  val: number
}>

export type Ids = TypeTag<'Ids', {
  val: Uuid
}>

export type RangeFilterBound = Absolute | Time | Ids

export const range_filter_bound_eq = (lhs: RangeFilterBound, rhs: RangeFilterBound): boolean =>
  lhs.type == rhs.type &&
  ((): boolean => {
    switch (lhs.type) {
      case "Absolute": return lhs.val == rhs.val
      case "Time": return lhs.val == rhs.val
      case "Ids": return lhs.val == rhs.val
    }
  })()

export interface RangeFilter {
  begin?: RangeFilterBound
  end?: RangeFilterBound
}

export const range_filter_eq = (lhs: RangeFilter, rhs: RangeFilter): boolean =>
  ((!lhs.begin && !rhs.begin) || lhs.begin && rhs.begin && range_filter_bound_eq(lhs.begin, rhs.begin)) &&
  ((!lhs.end && !rhs.end) || lhs.end && rhs.end && range_filter_bound_eq(lhs.end, rhs.end))

export interface Query {
  range_filter: RangeFilter
  event_filters: EventFilter<EventFilterType>[][]
  collection: Collection
}

export const query_eq = (lhs: Query, rhs: Query): boolean =>
  range_filter_eq(lhs.range_filter, rhs.range_filter) &&
  collection_eq(lhs.collection, rhs.collection) &&
  lhs.event_filters.every((a) => rhs.event_filters.some((b) => a.every((c) => b.some((d) => event_filter_eq(c, d)))))
