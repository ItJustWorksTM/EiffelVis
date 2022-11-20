<!-- svelte-ignore a11y-missing-attribute -->
<script lang="ts">
    import type { FilterInput, TemperateFilterArray } from "../uitypes";
    import LineInputList from "./LineInputList.svelte";
    const filter_types: string[] = ["ID", "Type", "Source", "Host", "Tag"];

    let selected = filter_types[0];
    let inputValue = "";
    export let tempFilterArray: TemperateFilterArray = [];
    const add_filter_to_TempFilterArray = () => {
        let newFilter = <FilterInput>{
            active: true,
            isWildCard: true,
            exclude: false,
            filterField: selected,
            value: inputValue,
        };
        tempFilterArray = [...tempFilterArray, newFilter];
        inputValue = "";
    };
</script>

<div class="w-full h-auto">
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
