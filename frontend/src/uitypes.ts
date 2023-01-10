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
    ids: EventFilter<Id>;
    tags: EventFilter<Tag>;
    types: EventFilter<Type>;
    sourcehosts: EventFilter<SourceHost>;
    sourcenames: EventFilter<SourceName>;
}

export const empty_fixed_event_filters = (): FixedEventFilters => {
    return {
        ids: {
            rev: false,
            pred: { type: 'Id', ids: [] },
        },
        tags: {
            rev: false,
            pred: { type: 'Tag', tags: [] },
        },
        types: {
            rev: false,
            pred: { type: 'Type', names: [] },
        },
        sourcehosts: {
            rev: false,
            pred: { type: 'SourceHost', hosts: [] },
        },
        sourcenames: {
            rev: false,
            pred: { type: 'SourceName', names: [] },
        },
    };
};

export interface FixedQuery {
    range_filter: RangeFilter;
    event_filters: FixedEventFilters[];
    collection: Collection;
}

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

                const push_if = (arr: any[], obj: any) => {
                    if (arr.length > 0) ret.push(obj);
                };

                push_if(v.ids.pred.ids, v.ids);
                push_if(v.tags.pred.tags, v.tags);
                push_if(v.types.pred.names, v.types);
                push_if(v.sourcehosts.pred.hosts, v.sourcehosts);
                push_if(v.sourcenames.pred.names, v.sourcenames);

                return ret;
            })
            .filter(arr => arr.length > 0),
        collection,
    };
};
