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
    <LineInputList bind:values={tempFilterArray} />
    <select class="select select-primary" bind:value={selected}>
        {#each filter_types as type}
            <option>{type}</option>
        {/each}
    </select>
    <input
        type="text"
        placeholder={selected}
        bind:value={inputValue}
        class="input input-sm input-bordered w-32 h-8"
    />
    <button
        class="btn btn-xs"
        on:click={() => add_filter_to_TempFilterArray()}
        disabled={inputValue == ""}>+</button
    >
</div>

<style lang="postcss" global>
    .hidden {
        display: none;
    }
    .select.select-primary {
        min-height: 2rem;
        padding-left: 0.6rem;
        padding-right: 1.7rem;
        margin-right: 0.5rem;
        height: 2rem;
    }
</style>
