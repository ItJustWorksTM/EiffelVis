<script lang="ts">
  import { onMount } from "svelte";
  import G6, { Graph, IG6GraphEvent, Item, Node } from "@antv/g6";
  import type { TimeBarData } from "../uitypes";
  import { createEventDispatcher } from "svelte";
  import { interactiveMode} from '../store';

  const dispatch = createEventDispatcher();
  const graph_translation: number = 50;

  export let options = {};
  export let data = {};

    let nodePoint:number = 0; // variable to retrieve x point of node

    let container: HTMLElement;
    let graph: Graph | null;
    let timeBarData: TimeBarData[] = [];
    export const reset = () => {
      graph?.changeData({});
      timeBarData = [];
      graph?.render();
      dispatch("nodeselected", null);
    };
      // This is a hack to get the graph to render the entire window width
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

     //non-InteractiveMode function to calculate how much translation the graph should make
    export const nonInteractiveMode = (e:any , modeDisabled:boolean) =>{
        if(modeDisabled == true){
            graph.translate(0,0); // to disable nonInteractive mode
        }
        else{
            let oldNodePoint:number = nodePoint; // give initial value of nodepoint to new variable
            let scrollDistance:number  = 0; // variable to calculate how far apart the two latest nodes on the graph are
            let scrollDistanceWithRatio:number = 0; // variable to calculate scroll distance variable multiplied with zoom ratio of graph
            let zoomRatio:number = graph.getZoom(); 
            nodePoint = e.x - container.scrollWidth;// grab latest node on the graph and find its point
            if(nodePoint != oldNodePoint){ // find out if latest node has moved to new horizontal position  
              scrollDistance = nodePoint - oldNodePoint; // if it has moved, find out how far it is from the nearest node to the left
              scrollDistanceWithRatio = scrollDistance*zoomRatio // multiple with zoom ratio for accurate graph translation
              graph.translate(-scrollDistanceWithRatio,0); // negate distance to indicate left translation
            }
        }
    }


    export const push = (ev: any) => {
    ev.date = String(ev.time);
    graph.addItem("node", ev, false, false);
    for (const edge of ev.edges) {
      graph.addItem("edge", {
        source: ev.id,
        target: edge.target,
        label: edge.type,
      }); // the type of link is connected to the label of the edge here.
    }
    edgesToBack(ev); // put all edges attached to the node behind the nodes (needed when using groupByTypes: false);
    timeBarData.push({
      date: ev.date,
      value: "1",
    });
  };
  /**
   * Helper method that will rearrange the order of items on the z-index (edges behind the nodes)
   * To avoid iterating through the whole graph and update, we use this helper method inside the push method.
   * This allows to only manipulate the nodes newly pushed into the graph.
   * @param event containing a node and its edges.
   */
  const edgesToBack = (event) => {
    const node: Item = graph.findById(event.id);
    if (node instanceof Node) {
      const edges = node.getEdges();
      for(const edge of edges){
        edge.toBack();
      }
    }
    graph.paint();
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
      itemTypes: ["node"],
      // custom the tooltip's content
      getContent: (e) => {
        const outDiv = document.createElement("div"); // create a new div to contain the info within the tootip.
        if (e.item.getType() === "node") {
          outDiv.innerHTML = `<h4>` + getNodeLocalTime(e) + `</h4>`; //TODO: only works for nodes. Should differenciate types of item (node or edge and give different info. Problem: edges don't contain causes ATM)
        }
        return outDiv;
      },
    });
    // Method that returns the local time contained in a node. The path to the time has been checked so it corresponds to the time when the node has been received by the rabbitMQ broker.
    const getNodeLocalTime = (e: any) => {
      let time = new Date(e.item._cfg.model.time); // create a new date from the timestamp in the node
      return time.toLocaleTimeString(); // return the converted date to local time with a precision to the second.
    };
    /**
     * Method that takes a node and highlights the edges, show the link type and activate the tool tip of the node with the time of the Eiffel event's creation
     * @param node of type Node
     */
    const showRelations = (node) => {
          // check if item is a Node to be able to access the getEdges() method.
          const edges = node.getEdges();
          edges.forEach((edge) => {
            edge.toFront(); // put edge on top of the nodes (to see lables)
            graph.updateItem(edge, {
              //update the edges of the node (used here to style labels and edge)
              labelCfg: {
                style: {
                  fillOpacity: 1, // change the opacity to 1(make it visible), as the default opacity is set to 0(invisible).
                },
              },
              style: {
                opacity: 1,
                lineWidth: 1.5,
              },
            });
          });
    }
    /**
     * Method that takes a node:event and undoes the effect of showRelations()
     * @param node of type Node 
     */
    const hideRelations = (node: Node) => {
          const edges = node.getEdges();
          edges.forEach((edge) => {
            edge.toBack(); // put edge back behond the node
            graph.updateItem(edge, {
              labelCfg: {
                style: {
                  fillOpacity: 0, // make the link lable invisible again, as the mouse moves away from the node
                },
              },
              style: {
                opacity: 0.15,
                lineWidth: 1,
              },
            });
          });
    }
    onMount(() => {
      if (graph) {
        graph.destroy();
      }
      graph = new G6.Graph({
        ...options,
        container,
        plugins: [tooltip], // add tooltip as plugin to the graph.
      });
      graph.on("nodeselectchange", (e) => dispatch("nodeselected", e));
      // Listeners that manipulates the nodes when they are hovered.
      graph.on("node:mouseenter", (e) => {
        if (e.item instanceof Node){
          showRelations(e.item);
          $interactiveMode= true;
          nonInteractiveMode(e,$interactiveMode);
        }
      });
      graph.on("node:mouseleave", (e) => {
        if (e.item instanceof Node){
          hideRelations(e.item);
        }
        
      });
      // Enable keyboard manipulation
      graph.on("keydown", (e: IG6GraphEvent) => {
        let weight: Function = (k1: string, k2: string) =>
          e.key == k1 ? -1 : e.key == k2 ? 1 : 0;
        graph.translate(
          weight("ArrowRight", "ArrowLeft") * graph_translation,
                    weight("ArrowDown", "ArrowUp") * graph_translation,
                )
        })         
        
        // deactivate on graph interactions such as drag ,mouseenter and node click   
        graph.on("canvas:drag", (e:IG6GraphEvent) => { 
                $interactiveMode= true;
                nonInteractiveMode(e,$interactiveMode);
            })
        
        graph.on("node:click", (e:IG6GraphEvent) => { 
              $interactiveMode= true;
              nonInteractiveMode(e,$interactiveMode);
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
  reset();
</script>

<svelte:window on:resize={resizeGraph} />

<div bind:this={container} class="container" >
  </div>

<style global>
  .container {
    height: 100%;
  }
  .g6TimeBar {
    background: rgb(33, 33, 32);
    border-radius: 20px;
    position: absolute !important;
    left: 45%; 
    bottom: 80px;
    z-index: 0;
  }

  .g6tooltip {
    background-color: rgb(32, 33, 33, 0.8);
    border-radius: 8px;
    align-items: center;
    border-color: #555555;
    border-width: 1px;
    box-shadow: rgb(35, 34, 34) 2px 2px 2px;
    align-items: center;
    font-size: 0.875rem;
    color: #ffffff;

  }
</style>
