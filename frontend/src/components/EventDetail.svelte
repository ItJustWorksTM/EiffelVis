<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import type { FullEvent } from "../apidefinition";

    const dispatch = createEventDispatcher();

    export let selected_node: FullEvent | null;
</script>

<div>
    <p>Time: {selected_node?.meta.time}</p>
    <p>Type: {selected_node?.meta.type}</p>
    <p>Host: {selected_node?.meta.source.host}</p>
    <p>Source: {selected_node?.meta.source.name}</p>
    <p>
        Tags: {selected_node?.meta.tags ? selected_node?.meta.tags : "n/a"}
    </p>
    <div
        data-tip="Use this node as the graph root"
        class="tooltip tooltip-neutral w-full"
    >
        <button
            class="btn btn-xs btn-accent font-mono w-full select-all"
            on:click={() => dispatch("useroot")}
            >{selected_node?.meta.id}</button
        >
    </div>
</div>
