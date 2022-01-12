<script lang="ts">
    import { onMount } from "svelte";
    import G6, { Graph, GraphData } from "@antv/g6";
    import type { TimeBarData } from "../apidefinition";

    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();

    export let options = {};
    export let data = {};

    let container: HTMLElement;

    let graph: Graph | null;
    let timeBarData:TimeBarData[] = [];

    export const reset = () => {
        graph?.changeData({});
        timeBarData = [];
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
        ev.date = String(ev.time)
        graph.addItem("node", ev, false, false);
        for (const target of ev.edges) {
            graph.addItem("edge", { source: ev.id, target });
        }

        timeBarData.push({
            date: ev.date,
            value: "1"
        });
    };

    export const updateTimeBar = (timeBarEnabled:boolean) => {
        graph.removePlugin(graph.get("plugins")[0])
        if(!timeBarEnabled){
          //TO-DO Reset the graph if wanted later
        }
        else{
            graph!.addPlugin(new G6.TimeBar({
          className: 'g6TimeBar',
          x: 0,
          y: 0,
          width: 500,
          height: 110,
          padding: 10,
          type: 'trend',
          changeData: false,
          trend: {
            data: timeBarData,
            smooth: true,
          },
          tick:{
            tickLabelFormatter: (timeBarData: any) => {
                return "";
            },
            /*
            tickLabelStyle: {
                
            },
            */
            tickLineStyle:{
                fill: '#f28c18'
            }
          },
          slider: {
            backgroundStyle: {
              fill: '#131616',
            },
            foregroundStyle: {
              fill: '#ffffff',
            },
            handlerStyle:{
                style:{
                    fill: '#f28c18',
                    stroke: '#f28c18'
                }
            }
          },
          controllerCfg: {
            fill: '#131616',
            stroke: '#131616',
            timePointControllerText: ' Point',
            timeRangeControllerText: ' Point',
          },
        }));
        }
    }

    onMount(() => {
        if (graph) {
            graph.destroy();
        }

        graph = new G6.Graph({
            ...options,
            container,
        });

        graph.on("nodeselectchange", (e) => dispatch("nodeselected", e));

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

<div bind:this={container} class="fuck" />

<style global>
    .fuck {
        height: 100%;
    }
    .g6TimeBar {
        background: #131616;
        border-radius: 20px;
        position: absolute !important;
        left:35%;
        bottom: 80px;
    }
</style>
