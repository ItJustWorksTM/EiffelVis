<!-- svelte-ignore a11y-missing-attribute -->
<script lang="ts">
<<<<<<< HEAD
    import type { FilterInput, TemperateFilterArray } from "../uitypes";
    import LineInputList from "./LineInputList.svelte";
    const filter_types: string[] = ["ID", "Type", "Source", "Host", "Tag"];
=======
    import {
        EventFilter,
        Id,
        SourceHost,
        SourceName,
        string_compare_default,
        Tag,
        Type,
    } from "../apidefinition";

    import LineInputList from "./LineInputList.svelte";
    import Input from "./TextInput.svelte";
>>>>>>> bfe06c7 (backend: allow client to somewhat control string matching)

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

<<<<<<< HEAD
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
=======
<div class="w-full h-full">
    <div class="tabs tabs-boxed">
        {#each filter_types as type}
            <a
                class="tab"
                class:tab-active={active_filter == type}
                on:click={() => (active_filter = type)}>{type}</a
            >
        {/each}
    </div>
    <div class:hidden={active_filter != "Id"}>
        <label
            class="cursor-pointer label"
            class:hidden={ids.pred.ids.length == 0}
        >
            <span class="label-text">Reversed</span>
            <input
                type="checkbox"
                class="toggle toggle-primary"
                bind:checked={ids.rev}
            />
        </label>
        <LineInputList bind:values={ids.pred.ids} let:index={i}>
            <Input placeholder={"uuid"} bind:value={ids.pred.ids[i]}/>
        </LineInputList>
        <button
            class="btn btn-xs w-full"
            on:click={() => (ids.pred.ids = [...ids.pred.ids, ""])}>+</button
        >
    </div>

    <div class:hidden={active_filter != "Type"}>
        <label
            class="cursor-pointer label"
            class:hidden={types.pred.names.length == 0}
        >
            <span class="label-text">Reversed</span>
            <input
                type="checkbox"
                class="toggle toggle-primary"
                bind:checked={types.rev}
            />
        </label>
        <LineInputList bind:values={types.pred.names} let:index={i}>
            <Input placeholder={"type name"} bind:value={types.pred.names[i].value} />
        </LineInputList>
        <button
            class="btn btn-xs w-full"
            on:click={() => (types.pred.names = [...types.pred.names, string_compare_default()])}
            >+</button
        >
    </div>

    <div class:hidden={active_filter != "Source"}>
        <label
            class="cursor-pointer label"
            class:hidden={sourcenames.pred.names.length == 0}
        >
            <span class="label-text">Reversed</span>
            <input
                type="checkbox"
                class="toggle toggle-primary"
                bind:checked={sourcenames.rev}
            />
        </label>
        <LineInputList bind:values={sourcenames.pred.names} let:index={i}>
            <Input placeholder={"source name"} bind:value={sourcenames.pred.names[i].value} />
        </LineInputList>
        <button
            class="btn btn-xs w-full"
            on:click={() =>
                (sourcenames.pred.names = [...sourcenames.pred.names, string_compare_default()])}
            >+</button
        >
    </div>

    <div class:hidden={active_filter != "Host"}>
        <label
            class="cursor-pointer label"
            class:hidden={sourcehosts.pred.hosts.length == 0}
        >
            <span class="label-text">Reversed</span>
            <input
                type="checkbox"
                class="toggle toggle-primary"
                bind:checked={sourcehosts.rev}
            />
        </label>
        <LineInputList values={sourcehosts.pred.hosts} let:index={i}>
            <Input placeholder={"host name"} bind:value={sourcehosts.pred.hosts[i].value} />
        </LineInputList>
        <button
            class="btn btn-xs w-full"
            on:click={() =>
                (sourcehosts.pred.hosts = [...sourcehosts.pred.hosts, string_compare_default()])}
            >+</button
        >
    </div>

    <div class:hidden={active_filter != "Tag"}>
        <label
            class="cursor-pointer label"
            class:hidden={tags.pred.tags.length == 0}
        >
            <span class="label-text">Reversed</span>
            <input
                type="checkbox"
                class="toggle toggle-primary"
                bind:checked={tags.rev}
            />
        </label>
        <LineInputList bind:values={tags.pred.tags} let:index={i}>
            <Input placeholder={"tag name"} bind:value={tags.pred.tags[i].value} />
        </LineInputList>
        <button
            class="btn btn-xs w-full"
            on:click={() => (tags.pred.tags = [...tags.pred.tags, string_compare_default()])}
            >+</button
>>>>>>> bfe06c7 (backend: allow client to somewhat control string matching)
        >
    </div>
</div>

<style lang="postcss" global>
    .hidden {
        display: none;
    }
</style>
