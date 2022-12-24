<!-- svelte-ignore a11y-missing-attribute -->
<script lang="ts">
    import type {
        EventFilter,
        Id,
        SourceHost,
        SourceName,
        Tag,
        Type,
    } from "../apidefinition";
    import { FilterInput, FilterType, TemperateFilterArray } from "../uitypes";

    import LineInputList from "./LineInputList.svelte";

    const filter_types: string[] = ["ID", "Type", "Source", "Host", "Tag"];

    // TODO: Maybe just fixed filter?
    export let ids: EventFilter<Id>[];

    export let tags: EventFilter<Tag>[];
    export let types: EventFilter<Type>[];
    export let sourcehosts: EventFilter<SourceHost>[];
    export let sourcenames: EventFilter<SourceName>[];
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
    let selected = filter_types[0];
    let inputValue = "";
    let tempFilterArray: TemperateFilterArray;
    const add_filter_to_TempFilterArray = () => {
        let newFilter = <FilterInput>{
            active: true,
            isWildCard: true,
            exclude: false,
            filterField: selected,
            value: inputValue,
        };
        tempFilterArray = [...tempFilterArray, newFilter];
        add_tempfilter_to_query();
        console.log(ids);
        console.log(tags);
        console.log(types);
        console.log(sourcehosts);
        console.log(sourcenames);
        inputValue = "";
    };

    export const add_tempfilter_to_query = () => {
        tempFilterArray.forEach((filter) => {
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
    };
</script>

<div class="w-full h-full">
    <div>
        <LineInputList bind:values={tempFilterArray} />
        <select class="select select-primary max-w-xs" bind:value={selected}>
            {#each filter_types as type}
                <option>{type}</option>
            {/each}
        </select>
        <input
            type="text"
            placeholder={selected}
            bind:value={inputValue}
            class="input input-sm input-bordered basis-full"
        />
        <button
            class="btn btn-xs"
            on:click={() => add_filter_to_TempFilterArray()}
            disabled={inputValue == ""}>+</button
        >
    </div>
</div>

<style lang="postcss" global>
    .hidden {
        display: none;
    }
</style>
