<script lang="ts">
    import { onMount } from "svelte";
    import G6, { Graph } from "@antv/g6";

    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();

    export let options = {};
    export let data = {};

    let container: HTMLElement;

    let graph: Graph | null;

    export const reset = () => {
        graph?.data({});
        graph?.render();
        dispatch("nodeselected", null);
    };

    export const resizeGraph = () => {
        if (graph && container) {
            const width = Number(
                window.getComputedStyle(container).width.replace("px", "")
            );
            const height = Number(
                window.getComputedStyle(container).height.replace("px", "")
            );
            graph.changeSize(width, height);
        }
    };

    export const focusNode = (id: any) => {
        graph.focusItem(id);
    };

    export const push = (ev: any) => {
        graph.addItem("node", ev, false, false);
        for (const target of ev.edges) {
            graph.addItem("edge", { source: ev.id, target });
        }
    };

    onMount(() => {
        if (graph) {
            graph.destroy();
        }

        graph = new G6.Graph({
            ...options,
            container,
        });

        graph.on("nodeselectchange", (e) => dispatch("nodeselected", e));

        graph.data(data);
        graph.render();
        resizeGraph();

        return () => {
            graph.destroy();
        };
    });

    $: {
        if (data && graph) {
            graph.data(data);
            graph.render();
        }
    }
</script>

<svelte:window on:resize={resizeGraph} />

<div bind:this={container} class="fuck" />

<style global>
    .fuck {
        height: 100%;
    }
</style>
