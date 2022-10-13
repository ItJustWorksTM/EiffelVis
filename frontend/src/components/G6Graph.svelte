<script lang="ts">
    import { onMount } from "svelte";
    import G6, { Graph, GraphAnimateConfig, IG6GraphEvent } from "@antv/g6";
    import type { TimeBarData } from "../uitypes";
    import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();

    const graph_translation: number = 50;

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
    console.log(ev);
    ev.date = String(ev.time);
    graph.addItem("node", ev, false, false);
    for (const edge of ev.edges) {
      graph.addItem("edge", { source: ev.id, target: edge.target, label: edge.type }); // the type of link is connected to the label of the edge here. 
    }

    timeBarData.push({
      date: ev.date,
      value: "1",
    });
  };

  export const updateTimeBar = (timeBarEnabled: boolean) => {
    graph.removePlugin(graph.get("plugins")[1]); // changed index to 1 since the timebar is added after the tooltip
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

  // Declare the tooltip. Styling happens below in the .g6tooltip section
  // doc: https://g6.antv.vision/en/examples/tool/tooltip#tooltipPlugin 
  const tooltip = new G6.Tooltip({
  className: "g6tooltip",
  offsetX: 10,
  offsetY: 10,
  width: 10,
  height: 8,
  // the types of items that allow the tooltip show up
  itemTypes: ['node'],
  // custom the tooltip's content
  getContent: (e) => {
    const outDiv = document.createElement('div'); // create a new div to contain the info within the tootip. 

    if (e.item.getType() === 'node') {
        outDiv.innerHTML = `<h4>`+ getNodeLocalTime(e) +`</h4>`; //TODO: only works for nodes. Should differenciate types of item (node or edge and give different info. Problem: edges don't contain causes ATM)
    }
        return outDiv;
  },
});

// Method that returns the local time contained in a node. The path to the time has been checked so it corresponds to the time when the node has been received by the rabbitMQ broker. 
  const getNodeLocalTime = (e: any) => {
    let time = new Date(e.item._cfg.model.time); // create a new date from the timestamp in the node
    return time.toLocaleTimeString();            // return the converted date to local time with a precision to the second. 
  }


  const clearAllStates = (e: any) =>{
    graph.getNodes().forEach(function (node) {
    graph.clearItemStates(node);
    });
    // this was causing an error  
    //graph.getEdges().forEach(function (edge) { 
    //graph.clearItemStates(edge);
   // });
    graph.paint();
  }

  onMount(() => {
    if (graph) {
      graph.destroy();
    }

    graph = new G6.Graph({
      ...options,
      container,
      plugins: [tooltip]  // add tooltip as plugin to the graph. 
    });

        graph.on("nodeselectchange", (e) => dispatch("nodeselected", e));


           // Listeners that highlight the nodes when they are hovered.
    graph.on("node:mouseenter", (e) => { //test
        const item = e.item;
        graph.getNodes().forEach(function (node) {
            graph.clearItemStates(node);
            graph.setItemState(node, 'dark', true);
            graph.setItemState(node, 'hover', true); //to have the hover effect fully functioning
        });
        graph.setItemState(item, 'dark', false);
        graph.setItemState(item, 'highlight', true);
        graph.getEdges().forEach(function (edge) {
            if (edge.getSource() === item) {
            graph.setItemState(edge.getTarget(), 'dark', false);
            graph.setItemState(edge.getTarget(), 'highlight', true);
            graph.setItemState(edge, 'highlight', true);
            graph.setItemState(edge, 'dark', false);         
            edge.toFront();
            } else if (edge.getTarget() === item) {
            graph.setItemState(edge.getSource(), 'dark', false);
            graph.setItemState(edge.getSource(), 'highlight', true);
            graph.setItemState(edge, 'dark', false);
            graph.setItemState(edge, 'highlight', true);
            edge.toFront();
            } else {
            graph.setItemState(edge, 'highlight', false);
            graph.setItemState(edge, 'dark', true);
            }
             graph.updateItem(edge, {
                stateStyles:{
                    highlight:{
                        'edge-label': {
                            opacity: 1
                        }
                    },
                    dark:{
                        'edge-label': {
                            opacity: 0
                        }
                    }
            }} 
        )//end of updateItem
    });
});
      
graph.on('node:mouseleave', clearAllStates);
graph.on('canvas:click', clearAllStates);
graph.on("node:mouseleave", (e) => {
    graph.getNodes().forEach(function (node) {
            graph.clearItemStates(node);
            graph.setItemState(node, 'dark', true);
            graph.setItemState(node, 'hover', false); //to have the hover effect fully functioning
        });
      graph.clearItemStates(e.item);
      graph.getEdges().forEach(function (edge) {
      //  graph.clearItemStates(edge);
      graph.updateItem(edge, {
                stateStyles:{
                    highlight:{
                        labelCfg:{ 
                            opacity: 0
                        }
                    },
                    dark:{
                        'label':{ // this should be like this.  when I changed it, it got buggy
                            opacity: 0
                        }
                }
            }
        } 
        )//end of updateItem
        });
     });
        // Enable keyboard manipulation
        graph.on("keydown", (e: IG6GraphEvent) => {
            let weight: Function = (k1: string, k2: string) => e.key == k1 ? -1 : e.key == k2 ? 1 : 0
                graph.translate(
                    weight("ArrowRight", "ArrowLeft") * graph_translation,
                    weight("ArrowDown", "ArrowUp") * graph_translation,
                )
        })
        
         //TODO : Add a listener when entering and exiting the nodes
         graph.on("node:mouseenter", (e) => dispatch("nodeHovered", e));  // emit an event when the node is entered with the mouse



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
  reset();

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

  .g6tooltip {
    background-image: linear-gradient(to right, rgb(33, 33, 33, 0.3), rgb(100, 100, 100));
    border-radius: 8px;
    align-items: center;
    border-color: rgb(83, 83, 83);
    border-width: 1px;
    padding: 0.5px;
    box-shadow: rgb(61, 61, 61) 2px 4px 12px;
    height: 30px; 
    width :fit-content; 
    text-shadow:2px 2px #444a5a;         
    color: #fff;
    font-family:'Major Mono Display';
  }

</style>
