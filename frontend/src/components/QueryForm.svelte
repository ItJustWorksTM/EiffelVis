<script lang="ts">
    import type { RangeFilterBound } from "../apidefinition";
    import type { FixedQuery } from "../uitypes";
    import FilterWidget from "./FilterWidget.svelte";

    const range_modes: string[] = ["None", "Time", "Absolute", "Ids"];
    const collection_modes: ("Forward" | "AsRoots")[] = ["Forward", "AsRoots"];

    let begin_mode: string = "None";
    let end_mode: string = "None";

    export let query: FixedQuery;
    export let filterWidget;

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
    {#each query.event_filters as filter, i}
        <div
            tabindex="0"
            class="grow collapse w-full border rounded-box border-base-300 collapse-arrow"
        >
            <input type="checkbox" />
            <div class="collapse-title text-base font-medium">
                {`Filter ${i}`}
            </div>
            <div class="collapse-content">
                <FilterWidget
                    bind:ids={filter.ids}
                    bind:tags={filter.tags}
                    bind:types={filter.types}
                    bind:sourcehosts={filter.sourcehosts}
                    bind:sourcenames={filter.sourcenames}
                    bind:this={filterWidget}
                />
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
