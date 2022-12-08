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
    export let ids: EventFilter<Id>;
    export let tags: EventFilter<Tag>;
    export let types: EventFilter<Type>;
    export let sourcehosts: EventFilter<SourceHost>;
    export let sourcenames: EventFilter<SourceName>;
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
        inputValue = "";
    };

    export const add_tempfilter_to_query = () => {
        tempFilterArray.forEach((filter) => {
            if (filter.active) {
                switch (filter.filterField) {
                    case FilterType.ID:
                        ids.pred.ids = [...ids.pred.ids, filter.value];
                        break;
                    case FilterType.Type:
                        types.pred.names = [
                            ...types.pred.names,
                            {
                                lower_case: filter.isWildCard,
                                partial: filter.isWildCard,
                                value: filter.value,
                            },
                        ];
                        break;
                    case FilterType.Source:
                        sourcenames.pred.names = [
                            ...sourcenames.pred.names,
                            {
                                lower_case: filter.isWildCard,
                                partial: filter.isWildCard,
                                value: filter.value,
                            },
                        ];
                        break;
                    case FilterType.Host:
                        sourcehosts.pred.hosts = [
                            ...sourcehosts.pred.hosts,
                            {
                                lower_case: filter.isWildCard,
                                partial: filter.isWildCard,
                                value: filter.value,
                            },
                        ];
                        break;
                    case FilterType.Tag:
                        tags.pred.tags = [
                            ...tags.pred.tags,
                            {
                                lower_case: filter.isWildCard,
                                partial: filter.isWildCard,
                                value: filter.value,
                            },
                        ];
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
