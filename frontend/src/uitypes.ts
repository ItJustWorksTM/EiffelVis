import type {
    EventFilter,
    Id,
    Type,
    SourceHost,
    SourceName,
    Tag,
    RangeFilter,
    Collection,
    Query,
} from './apidefinition';

export interface TimeBarData {
    date: string;
    value: string;
}

export interface FixedEventFilters {
    ids: EventFilter<Id>[]
    tags: EventFilter<Tag>[]
    types: EventFilter<Type>[]
    sourcehosts: EventFilter<SourceHost>[]
    sourcenames: EventFilter<SourceName>[]
}

export const empty_fixed_event_filters = (): FixedEventFilters => {
    return {
        ids: [],
        tags: [],
        types: [],
        sourcehosts: [],
        sourcenames: [],
    }
}

export interface FixedQuery {
    range_filter: RangeFilter;
    event_filters: FixedEventFilters[];
    collection: Collection;
    range_filter: RangeFilter;
    event_filters: FixedEventFilters[];
    collection: Collection;
}

export const fixed_query_to_norm = ({
    range_filter,
    collection,
    event_filters,
}: FixedQuery): Query => {
    export const fixed_query_to_norm = ({
        range_filter,
        collection,
        event_filters,
    }: FixedQuery): Query => {
        return {
            range_filter,
            event_filters: event_filters
                .map(v => {
                    const ret = [];

                    const push_if = (arr: any[]) => {
                        if (arr.length > 0) {
                            arr.map((filter) => {
                                ret.push(filter)

                            })
                        }

                    };

                    push_if(v.ids);
                    push_if(v.tags);
                    push_if(v.types);
                    push_if(v.sourcehosts);
                    push_if(v.sourcenames);

                    return ret;
                }).filter((arr) => arr.length > 0),
            collection
        }
    }
    export enum FilterType {
        ID = "ID",
        Host = "Host",
        Source = "Source",
        Tag = "Tag",
        Type = "Type"
    }

    export interface FilterInput {
        active: boolean
        exclude: boolean
        isWildCard: boolean
        filterField: FilterType
        value: string
    }

    export type TemperateFilterArray = FilterInput[]