<script lang="ts">
    import type {
        EventFilter,
        Id,
        RangeFilterBound,
        SourceHost,
        SourceName,
        Tag,
        Type,
    } from "../apidefinition";
    import {
        empty_fixed_event_filters,
        FilterType,
        FixedQuery,
        TemperateFilterArray,
    } from "../uitypes";
    import FilterWidget from "./FilterWidget.svelte";

    const range_modes: string[] = ["None", "Time", "Absolute", "Ids"];
    const collection_modes: ("Forward" | "AsRoots")[] = ["Forward", "AsRoots"];

    let begin_mode: string = "None";
    let end_mode: string = "None";

    export let query: FixedQuery;
    export let event_filters_sets: TemperateFilterArray[];
    let filterWidget;
    let ids: EventFilter<Id>[] = [];
    let tags: EventFilter<Tag>[] = [];
    let types: EventFilter<Type>[] = [];
    let sourcehosts: EventFilter<SourceHost>[] = [];
    let sourcenames: EventFilter<SourceName>[] = [];
    let id: EventFilter<Id> = { rev: false, pred: { type: "Id", ids: [] } };
    let tag: EventFilter<Tag> = { rev: false, pred: { type: "Tag", tags: [] } };
    let type: EventFilter<Type> = {
        rev: false,
        pred: { type: "Type", names: [] },
    };
    let sourcehost: EventFilter<SourceHost> = {
        rev: false,
        pred: { type: "SourceHost", hosts: [] },
    };
    let sourcename: EventFilter<SourceName> = {
        rev: false,
        pred: { type: "SourceName", names: [] },
    };
    let select_filter_set = [];
    $: {
        add_tempfilter_to_query(select_filter_set);
    }
    function add_tempfilter_to_query(value) {
        if (value.length > 0) {
            console.log("add filter to query");
            console.log(value);
            ids = [];
            types = [];
            tags = [];
            sourcehosts = [];
            sourcenames = [];
            value.forEach((filter) => {
                if (filter.active) {
                    switch (filter.filterField) {
                        case FilterType.ID:
                            id.rev = filter.exclude;
                            id.pred.ids[0] = filter.value;
                            ids = [...ids, id];
                            break;
                        case FilterType.Type:
                            type.rev = filter.exclude;
                            type.pred.names[0] = {
                                lower_case: filter.isWildCard,
                                partial: filter.isWildCard,
                                value: filter.value,
                            };
                            types = [...types, type];
                            break;
                        case FilterType.Source:
                            sourcehost.rev = filter.exclude;
                            sourcehost.pred.hosts[0] = {
                                lower_case: filter.isWildCard,
                                partial: filter.isWildCard,
                                value: filter.value,
                            };
                            sourcehosts = [...sourcehosts, sourcehost];
                            break;
                        case FilterType.Tag:
                            tag.rev = filter.exclude;
                            tag.pred.tags[0] = {
                                lower_case: filter.isWildCard,
                                partial: filter.isWildCard,
                                value: filter.value,
                            };
                            tags = [...tags, tag];
                            break;
                        case FilterType.Source:
                            sourcename.rev = filter.exclude;
                            sourcename.pred.names[0] = {
                                lower_case: filter.isWildCard,
                                partial: filter.isWildCard,
                                value: filter.value,
                            };
                            sourcenames = [...sourcenames, sourcename];
                            break;
                        default:
                            break;
                    }
                }
            });
            query.event_filters[0].ids = ids;
            query.event_filters[0].sourcehosts = sourcehosts;
            query.event_filters[0].sourcenames = sourcenames;
            query.event_filters[0].tags = tags;
            query.event_filters[0].types = types;
            console.log("query");
            console.log(query.event_filters);
        } else {
            query.event_filters = [];
            query.event_filters[0] = empty_fixed_event_filters();
        }
    }
    const mkk = (
        type: "Time" | "Absolute" | "Ids",
        val: string
    ): RangeFilterBound => {
        switch (type) {
            case "Time":
                return { type, val: parseInt(val) };
            case "Absolute":
                return { type, val: parseInt(val) };
            case "Ids":
                return { type, val: "" + val };
        }
    };

    const set_collection_mode = (mode: "Forward" | "AsRoots") => {
        query.collection.type = mode;
    };

    const set_start_range_mode = (mode: RangeFilterBound | null) =>
        (begin_mode = mode ? mode.type : "None");
    $: set_start_range_mode(query.range_filter.begin);

    const set_end_range_mode = (mode: RangeFilterBound | null) =>
        (end_mode = mode ? mode.type : "None");
    $: set_end_range_mode(query.range_filter.end);

    const floopydoop = (val: any) =>
        val != "None" ? mkk(val, val != "Ids" ? "0" : "") : null;

    const setfloop = (type: any, val: any) =>
        mkk(type, val.length == 0 ? "0" : val);

    const setmodebegin = (val: any) =>
        (query.range_filter.begin = floopydoop(val));
    const setmodeend = (val: any) => (query.range_filter.end = floopydoop(val));

    const setvalbegin = (val: any) =>
        (query.range_filter.begin = setfloop(
            query.range_filter.begin.type,
            val
        ));
    const setvalend = (val: any) =>
        (query.range_filter.end = setfloop(query.range_filter.end.type, val));
</script>

<div>
    {#each event_filters_sets as filter, i}
        <div
            tabindex="0"
            class="grow collapse w-full border rounded-box border-base-300 collapse-arrow"
        >
            <input type="checkbox" />
            <div class="collapse-title text-base font-medium">
                {`filters set ${i}`}
            </div>

            <div
                class="collapse-content"
                on:click={() => {
                    if (filter.length > 0) {
                        select_filter_set = filter;
                    } else {
                        select_filter_set = [];
                    }
                    console.log(select_filter_set);
                }}
            >
                <FilterWidget
                    bind:this={filterWidget}
                    bind:tempFilterArray={filter}
                />
                {#if i != 0}
                    <button
                        class="btn overflow-auto w-full mt-2"
                        on:click={() => {
                            event_filters_sets.splice(i, 1);
                            event_filters_sets = [...event_filters_sets];
                        }}
                    >
                        delet filter set
                    </button>
                {/if}
                <!-- <FilterWidget
                    bind:ids={filter.ids}
                    bind:tags={filter.tags}
                    bind:types={filter.types}
                    bind:sourcehosts={filter.sourcehosts}
                    bind:sourcenames={filter.sourcenames}
                    bind:this={filterWidget}
                /> -->
            </div>
        </div>
    {/each}
    <div class="form-control">
        <!-- svelte-ignore a11y-label-has-associated-control -->
        {#each [{ setmode: setmodebegin, setval: setvalbegin, bound: query.range_filter.begin, mode: begin_mode }, { setmode: setmodeend, setval: setvalend, bound: query.range_filter.end, mode: end_mode }] as { setmode, setval, bound, mode }}
            <label class="input-group input-group-sm mt-1">
                <select
                    class="select select-bordered select-sm"
                    value={mode}
                    on:input={(e) => setmode(e.target.value)}
                >
                    {#each range_modes as mode}
                        <option>{mode}</option>
                    {/each}
                </select>
                <input
                    type={bound?.type == "Ids" ? "text" : "number"}
                    disabled={!bound}
                    placeholder={"Begin"}
                    value={bound ? bound.val : ""}
                    on:input={(e) => setval(e.target.value)}
                    class="input input-bordered input-sm w-full"
                />
            </label>
        {/each}
    </div>
    <div class="btn-group w-full flex flex-row mt-2">
        {#each collection_modes as mode}
            <button
                class="btn btn-xs grow"
                class:btn-active={mode == query.collection.type}
                on:click={() => set_collection_mode(mode)}>{mode}</button
            >
        {/each}
    </div>
</div>
