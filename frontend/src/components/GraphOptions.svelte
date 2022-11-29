<script lang="ts">
    import { createEventDispatcher } from "svelte";

    import type { GraphSettings } from "../layout";

    const dispatch = createEventDispatcher();

    export let graph_options: GraphSettings;
</script>

<div class="p-3 flex flex-col w-full" >
    <h1 class="text-lg py-2">Graph Options</h1>
    <label class="input-group input-group-sm mt-1">
        <span
            class="border border-[rgba(255,255,255,0.2)] span w-1/2 bg-base-100"
            >Offset</span
        >
        <input
            type="number"
            bind:value={graph_options.offset}
            class="input input-bordered input-sm w-1/2 h-max"
        />
    </label>
    <label class="input-group input-group-sm mt-1">
        <span
            class="span border border-[rgba(255,255,255,0.2)] w-1/2 bg-base-100"
            >Time Collapse</span
        >
        <input
            type="number"
            bind:value={graph_options.time_diff}
            class="input input-bordered input-sm w-1/2 h-max"
        />
    </label>
    <label class="input-group input-group-sm mt-1">
        <span
            class="span border border-[rgba(255,255,255,0.2)] w-1/2 bg-base-100"
            >Y-axis Scaling</span
        >
        <input
            type="number"
            bind:value={graph_options.y_scale}
            class="input input-bordered input-sm w-1/2 h-max"
        />
    </label>
    <label class="input-group input-group-sm mt-1">
        <span
            class="span border border-[rgba(255,255,255,0.2)] w-1/2 bg-base-100"
            >X-axis Node Separation</span
        >
        <input
            type="number"
            bind:value={graph_options.x_sep}
            class="input input-bordered input-sm w-1/2 h-max"
        />
    </label>
    <label class="input-group input-group-sm mt-1">
        <span
            class="span border border-[rgba(255,255,255,0.2)] w-1/2 bg-base-100"
            >Y-axis Node Separation</span
        >
        <input
            type="number"
            bind:value={graph_options.y_sep}
            class="input input-bordered input-sm w-1/2 h-max"
        />
    </label>
    <label class="input-group input-group-sm mt-1">
        <span
            class="span border border-[rgba(255,255,255,0.2)] w-1/2 bg-base-100"
            >Hue (Node Color)</span
        >
        <input
            type="number"
            bind:value={graph_options.hue}
            class="input input-bordered input-sm w-1/2 h-max"
        />
    </label>
    <div class="btn-group w-full flex flex-row mt-2">
        <button
            class="btn btn-sm grow btn-primary"
            on:click={() => dispatch("apply")}>Update Graph</button
        >
        <button
            class="btn btn-sm grow btn-primary btn-outline"
            on:click={() => dispatch("reset")}>Reset Default</button
        >
    </div>
</div>
