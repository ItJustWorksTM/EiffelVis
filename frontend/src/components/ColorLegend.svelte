<script lang="ts">
    export let styles;
    import G6, { Graph } from "@antv/g6";
    import { onMount } from "svelte";
    import { createEventDispatcher } from "svelte";
    import { defaultNode } from "../layout";
    const options = {
        container: "mountNode",
        width: 280,
        height: 220,
        workerEnabled: true,
        nodeSize: 20,
    };
    const dispatch = createEventDispatcher();

    let data = {};
    let array = [];
    const loadNodeWithStyle = () => {
        styles.map((event) => {
            let node = {
                id: event[0],
                size: 10,
                type: event[1].Shape,
                style: {
                    fill: event[1].Color,
                    stroke: event[1].Color,
                },
                label: event[1].Acronym,
                labelCfg: {
                    style: {
                        fill: event[1].Color,
                    },
                    position: "bottom",
                    offset: 10,
                },
            };
            array.push(node);
        });
    };
    const loadDefaultNode = () => {
        let default_Node = {
            id: "defaultNode",
            size: 10,
            type: defaultNode.shape,
            style: {
                fill: defaultNode.color,
                stroke: defaultNode.color,
            },
            label: defaultNode.type,
            labelCfg: {
                style: {
                    fill: defaultNode.color,
                },
                position: "bottom",
                offset: 10,
            },
        };
        array.push(default_Node);
    };
    loadNodeWithStyle();
    loadDefaultNode();
    data = { nodes: array };
    let container: HTMLElement;
    let graph: Graph | null;
    export const reset = () => {
        graph?.changeData({});
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
        console.log(id);
    };

    onMount(() => {
        if (graph) {
            graph.destroy();
        }
        graph = new G6.Graph({
            ...options,
            container,
        });
        graph.on("nodeselectchange", (e) => {
            dispatch("nodeselected", e), console.log(e);
        });
        graph.changeData(data);
        resizeGraph();

        return () => {
            graph.destroy();
        };
    });
</script>

<div class="flex flex-col w-full">
<<<<<<< HEAD
<<<<<<< HEAD
    <div class="bg-base-200 text-base-content grid h-10 place-items-center">
=======
    <div class="bg-base-100 text-base-content grid h-10 place-items-center">
>>>>>>> ebdd8b1 (Frontend: Graph Legend Update (#124))
=======
    <div class="bg-base-200 text-base-content grid h-10 place-items-center">
>>>>>>> db0a866 (FrontEnd: Re-factor the overall layout (#141))
        Legend
    </div>
    <div bind:this={container} class="legend" />
</div>

<style>
    .legend {
        margin: 10px;
        padding: 10px;
    }
</style>
