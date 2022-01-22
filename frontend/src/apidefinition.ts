/* eslint-disable camelcase */
// Interally tagged type for types that are part of a enum
type TypeTag<K, T> = K & { type: T }

export type Uuid = string

export interface Event {
  id: Uuid
  time: number
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
  names: string[]
}
export type Type = TypeTag<_Type, 'Type'>

interface _SourceHost {
  hosts: string[]
}
export type SourceHost = TypeTag<_SourceHost, 'SourceHost'>

interface _SourceName {
  names: string[]
}
export type SourceName = TypeTag<_SourceName, 'SourceName'>

interface _Tag {
  tags: string[]
}
export type Tag = TypeTag<_Tag, 'Tag'>

interface _Id {
  ids: Uuid[]
}
export type Id = TypeTag<_Id, 'Id'>

export type Forward = TypeTag<_Forward, 'Forward'>
interface _Forward { }

export type AsRoots = TypeTag<_AsRoots, 'AsRoots'>
interface _AsRoots { }

export type EventFilterType = Type | Id | SourceHost | SourceName | Tag
export interface EventFilter<T> {
  rev: boolean
  pred: T
}
export type Collection = Forward | AsRoots
export type RangeFilterBound = Absolute | Time | Ids

export interface RangeFilter {
  begin?: RangeFilterBound
  end?: RangeFilterBound
}

export interface Query {
  range_filter: RangeFilter
  event_filters: EventFilter<EventFilterType>[][]
  collection: Collection
}

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

export const event_filter_eq = (lhs: EventFilter<EventFilterType>, rhs: EventFilter<EventFilterType>) =>
  lhs.rev == rhs.rev && event_filter_type_eq(lhs.pred, rhs.pred)

export const range_filter_bound_eq = (lhs: RangeFilterBound, rhs: RangeFilterBound): boolean =>
  lhs.type == rhs.type &&
  ((): boolean => {
    switch (lhs.type) {
      case "Absolute": return lhs.val == rhs.val
      case "Time": return lhs.val == rhs.val
      case "Ids": return lhs.val == rhs.val
    }
  })()

export const range_filter_eq = (lhs: RangeFilter, rhs: RangeFilter): boolean =>
  ((!lhs.begin && !rhs.begin) || lhs.begin && rhs.begin && range_filter_bound_eq(lhs.begin, rhs.begin)) &&
  ((!lhs.end && !rhs.end) || lhs.end && rhs.end && range_filter_bound_eq(lhs.end, rhs.end))

export const collection_eq = (lhs: Collection, rhs: Collection): boolean => lhs.type == rhs.type

export const query_eq = (lhs: Query, rhs: Query): boolean =>
  range_filter_eq(lhs.range_filter, rhs.range_filter) &&
  collection_eq(lhs.collection, rhs.collection) &&
  lhs.event_filters.every((a) => rhs.event_filters.some((b) => a.every((c) => b.some((d) => event_filter_eq(c, d)))))

export interface QueryRes {
  repl: string
  error?: string
}

export type ServerMessage = QueryRes | Event[]

export interface TimeBarData {
  date: string
  value: string
}

export interface FixedEventFilters {
  ids: EventFilter<Id>
  tags: EventFilter<Tag>
  types: EventFilter<Type>
  sourcehosts: EventFilter<SourceHost>
  sourcenames: EventFilter<SourceName>
}

export const empty_fixed_event_filters = (): FixedEventFilters => {
  return {
    ids: {
      rev: false,
      pred: { type: "Id", ids: [] },
    },
    tags: {
      rev: false,
      pred: { type: "Tag", tags: [] },
    },
    types: {
      rev: false,
      pred: { type: "Type", names: [] },
    },
    sourcehosts: {
      rev: false,
      pred: { type: "SourceHost", hosts: [] },
    },
    sourcenames: {
      rev: false,
      pred: { type: "SourceName", names: [] },
    },
  }
}

export interface FixedQuery {
  range_filter: RangeFilter
  event_filters: FixedEventFilters[]
  collection: Collection
}

export const fixed_query_to_norm = ({ range_filter, collection, event_filters }: FixedQuery): Query => {
  return {
    range_filter,
    event_filters: event_filters.map((v) => {
      const ret = []

      const push_if = (arr: any[], obj: any) => {
        if (arr.length > 0) ret.push(obj);
      };

      push_if(v.ids.pred.ids, v.ids);
      push_if(v.tags.pred.tags, v.tags);
      push_if(v.types.pred.names, v.types);
      push_if(v.sourcehosts.pred.hosts, v.sourcehosts);
      push_if(v.sourcenames.pred.names, v.sourcenames);

      return ret;
    }).filter((arr) => arr.length > 0),
    collection
  }
}