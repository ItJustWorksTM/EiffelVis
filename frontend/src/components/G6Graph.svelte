<script lang="ts">
    import { onMount } from "svelte";
    import G6, { Graph, GraphData, IG6GraphEvent } from "@antv/g6";
    import type { TimeBarData } from "../uitypes";
    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();

    const graph_translation = 50;

    export let options = {};
    export let data = {};

    let container: HTMLElement;

    let graph: Graph | null;
    let timeBarData: TimeBarData[] = [];

    export const reset = () => {
        graph?.changeData({});
        timeBarData = [];
        graph?.render();
        dispatch("nodeselected", null);
    };

    // This is a hack to get the graph to render properly
    // and not be cut off prematurely in width
    export const resizeGraph = () => {
        if (graph && container) {
            const width = Number(
                window.innerWidth
            );
            const height = Number(
                window.innerHeight
            );
            graph.changeSize(width, height);
        }
    };

    export const focusNode = (id: any) => {
        graph.focusItem(id);
    };

    export const push = (ev: any) => {
        ev.date = String(ev.time);
        graph.addItem("node", ev, false, false);
        for (const target of ev.edges) {
            graph.addItem("edge", { source: ev.id, target });
        }

        timeBarData.push({
            date: ev.date,
            value: "1",
        });
    };

    export const updateTimeBar = (timeBarEnabled: boolean) => {
        graph.removePlugin(graph.get("plugins")[0]);
        if (!timeBarEnabled) {
            //TO-DO Reset the graph if wanted later
        } else {
            graph!.addPlugin(
                new G6.TimeBar({
                    className: "g6TimeBar",
                    x: 0,
                    y: 0,
                    width: 500,
                    height: 110,
                    padding: 10,
                    type: "trend",
                    changeData: false,
                    trend: {
                        data: timeBarData,
                        smooth: true,
                    },
                    tick: {
                        tickLabelFormatter: (timeBarData: any) => {
                            return "";
                        },

                        tickLineStyle: {
                            fill: "#f28c18",
                        },
                    },
                    slider: {
                        backgroundStyle: {
                            fill: "#131616",
                        },
                        foregroundStyle: {
                            fill: "#ffffff",
                        },
                        handlerStyle: {
                            style: {
                                fill: "#f28c18",
                                stroke: "#f28c18",
                            },
                        },
                    },
                    controllerCfg: {
                        fill: "#131616",
                        stroke: "#131616",
                        timePointControllerText: " Point",
                        timeRangeControllerText: " Point",
                    },
                })
            );
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

        // Enable keyboard manipulation
        graph.on("keydown", (e: IG6GraphEvent) => {
            if (e.key === "ArrowRight") {
                graph.translate(-graph_translation, 0);
            } else if (e.key === "ArrowLeft") {
                graph.translate(graph_translation, 0);
            } else if (e.key === "ArrowUp") {
                graph.translate(0, graph_translation);
            } else if (e.key === "ArrowDown") {
                graph.translate(0, -graph_translation);
            }
        })

        graph.changeData(data);
        resizeGraph();

        return () => {
            graph.destroy();
        };
    });

    $: {
        if (data && graph) {
            graph.changeData(data);
        }
    }
</script>

<svelte:window on:resize={resizeGraph} />

<div bind:this={container} class="container" />

<style global>
    .container {
        height: 100%;
    }
    .g6TimeBar {
        background: #131616;
        border-radius: 20px;
        position: absolute !important;
        left: 35%;
        bottom: 80px;
        z-index: 0;
    }
</style>
