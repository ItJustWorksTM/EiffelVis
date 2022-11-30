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

    import LineInputList from "./LineInputList.svelte";

    const filter_types: string[] = ["Id", "Type", "Source", "Host", "Tag"];
    let active_filter: string = "Id";

    // TODO: Maybe just fixed filter?
    export let ids: EventFilter<Id>;
    export let tags: EventFilter<Tag>;
    export let types: EventFilter<Type>;
    export let sourcehosts: EventFilter<SourceHost>;
    export let sourcenames: EventFilter<SourceName>;
</script>

<div class="w-full h-full flex flex-col">
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
        <LineInputList placeholder={"uuid"} bind:values={ids.pred.ids} />
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
        <LineInputList
            placeholder={"type name"}
            bind:values={types.pred.names}
        />
        <button
            class="btn btn-xs w-full"
            on:click={() => (types.pred.names = [...types.pred.names, ""])}
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
        <LineInputList
            placeholder={"source name"}
            bind:values={sourcenames.pred.names}
        />
        <button
            class="btn btn-xs w-full"
            on:click={() =>
                (sourcenames.pred.names = [...sourcenames.pred.names, ""])}
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
        <LineInputList
            placeholder={"host name"}
            bind:values={sourcehosts.pred.hosts}
        />
        <button
            class="btn btn-xs w-full"
            on:click={() =>
                (sourcehosts.pred.hosts = [...sourcehosts.pred.hosts, ""])}
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
        <LineInputList placeholder={"tag name"} bind:values={tags.pred.tags} />
        <button
            class="btn btn-xs w-full"
            on:click={() => (tags.pred.tags = [...tags.pred.tags, ""])}
            >+</button
        >
    </div>
</div>

<style lang="postcss" global>
    .hidden {
        display: none;
    }
</style>
