<!-- svelte-ignore a11y-missing-attribute -->
<script lang="ts">
  import G6 from "@antv/g6";
  import { QueryStream, EiffelVisConnection } from "./eiffelvis";
  import { GraphSettings, StatefulLayout } from "./layout";
  import QueryForm from "./components/QueryForm.svelte";
  import EventDetail from "./components/EventDetail.svelte";
  import GraphOptions from "./components/GraphOptions.svelte";
  import ColorLegend from "./components/ColorLegend.svelte";
  import { get } from 'svelte/store';
  import { interactiveMode} from './store';

  import { query_eq } from "./apidefinition";
  import {
    empty_fixed_event_filters,
    FixedQuery,
    fixed_query_to_norm,
  } from "./uitypes";
  import { deep_copy } from "./utils";
  import G6Graph from "./components/G6Graph.svelte";
  import config from "./config.json";

  let graph_elem: G6Graph | null;
  export let connection: EiffelVisConnection;
  let active_stream: QueryStream | null = null;
  let awaiting_query_request = false;
  let selected_node = null;
  let show_menu = false;
  let show_legend = true;
  let show_timebar = false;
  let customTheme = config.Theme.ColorBlind;
  let themeMap = new Map(Object.entries(customTheme));
  let legend = themeMap;
  $: styles = [...legend.entries()];
  let query_cache: { stream: QueryStream; query: FixedQuery }[] = [];
  let qhistory: FixedQuery[] = [];
  let current_query: FixedQuery = {
    range_filter: { begin: { type: "Absolute", val: -500 }, end: null },
    event_filters: [empty_fixed_event_filters()],
    collection: { type: "Forward" },
  };
  $: current_query_changed =
    qhistory.length > 0 &&
    !query_eq(
      fixed_query_to_norm(current_query),
      fixed_query_to_norm(qhistory[qhistory.length - 1])
    );
  let graph_options: GraphSettings = {
    offset: 0,
    time_diff: 1000,
    y_scale: 0.99,
    x_sep: 60,
    y_sep: 60,
    hue: 360,
  };
  $: {
    if (graph_elem) {
      // TODO: split up?
      graph_elem.resizeGraph();
      selected_node = null;
      submit_state_query();
    }
  }


                                                               // non-interactive mode variables
  let time = new Date();
  let show_message = false; 
  let dayToDisplay = null; 
  let dayLastEventRecieved = 0; 
  let monthLastEventRecieved = 0; 
  let yearLastEventRecieved = 0; 
  let recievedNewNode = false; 
  let currentDay = 0;
  let displayTime = null;
  


const displayInfoMessage= () =>{ //After 1 minute of no nodes recieved, a message is displayed. 
  let time = new Date();
  currentDay = time.getDate();
  if (currentDay = dayLastEventRecieved){
      dayToDisplay = "TODAY"; 
      
  }else if (currentDay - dayLastEventRecieved == 1){
    
      dayToDisplay = "YESTERDAY"; 
  }else if (currentDay - dayLastEventRecieved> 1){
      dayToDisplay = yearLastEventRecieved.toString() + "-"+ monthLastEventRecieved.toString()+ "-" +dayLastEventRecieved.toString();
  }


  
  if (recievedNewNode==false && dayToDisplay != null  ){
    show_message = true; 
    console.log("recieved no new node")
  }else{
    show_message = false;
  }
  
}

  let ms = 60000;
 let interval= setInterval( displayInfoMessage, ms);

 const startTimer = () =>{
  clearInterval(interval);
  interval= setInterval( displayInfoMessage, ms);
}

  const consume_query = async () => {
    const layout = new StatefulLayout();
    awaiting_query_request = true;
    const iter = await active_stream.iter();
    awaiting_query_request = false;
    graph_elem.reset();
    let once = true;
    for await (const event of iter) {
      layout.apply(event, graph_options);
      graph_elem.push(event);

      graph_elem.nonInteractiveMode(event,$interactiveMode);
      //every time a node is pushed to the graph the variables are updated
      recievedNewNode = true;
      show_message = false; 
      dayLastEventRecieved = time.getDate(); 
      monthLastEventRecieved = time.getMonth();
      yearLastEventRecieved = time.getFullYear();
      displayTime= time.toLocaleTimeString([], {hour: '2-digit', minute: '2-digit'})


       
      // TODO: Find a better way to do this
      if (once) {
        graph_elem.focusNode(event.id);
        once = false;
      }

      legend = layout.getNodeStyle();
    }
    recievedNewNode = false; 
    console.log("stoped recieving nodes")
    startTimer();
    
  };
  const submit_state_query = () => submit_query(current_query);
  const submit_query = (fquery: FixedQuery) => {
    const new_query = fixed_query_to_norm(fquery);
    active_stream = (() => {
      const cached = query_cache.find((v) =>
        query_eq(new_query, fixed_query_to_norm(v.query))
      );
      if (cached) {
        return cached.stream;
      } else {
        const ret = new QueryStream(connection, deep_copy(new_query));
        query_cache = [
          ...query_cache,
          { stream: ret, query: deep_copy(fquery) },
        ];
        return ret;
      }
    })();
    consume_query();
    qhistory = [...qhistory, deep_copy(fquery)];
    show_timebar = false;
    graph_elem.updateTimeBar(show_timebar);
  };
  const add_filter = () => {};
  // TODO: add loading for this
  const on_node_selected = async (e: any) => {
    if (e.detail?.target) {
      selected_node = await connection.fetch_node(
        e.detail.target._cfg.model.id
      );
    } else {
      selected_node = null;
    }
  };
  const use_selected_as_root = () => {
    current_query.collection = { type: "AsRoots" };
    current_query.range_filter = { begin: null, end: null };
    const filters = empty_fixed_event_filters();
    filters.ids.pred.ids = [selected_node.meta.id];
    current_query.event_filters = [filters];
    submit_state_query();
  };
  const reset_graph_options = () => {
    graph_options = {
      offset: 0,
      time_diff: 1000,
      y_scale: 0.99,
      x_sep: 60,
      y_sep: 60,
      hue: 360,
    };
    consume_query();
  };
  const toggleMenu = () => {
    if (show_legend) {
      toggleLegend();
    }
    show_menu = !show_menu;
  };
  const toggleLegend = () => {
    if (show_menu) {
      toggleMenu();
    }
    show_legend = !show_legend;
  };
  const options = {
    width: 400,
    height: 400,
    workerEnabled: false,
    fitView: true,
    fitViewPadding:[0,0,0,800],
    groupByTypes: false,  // enables to control z-index of items https://antv-g6.gitee.io/en/docs/manual/middle/elements/methods/elementIndex
    defaultEdge: {
      labelCfg: {
        position: 'center', 
        style:{           // default styling for the edge labels should come here https://g6.antv.vision/en/docs/manual/middle/elements/edges/defaultEdge
          fontSize: 10,
          fill: '#ffffff',
          fillOpacity: 0,
          shadowColor: "#151517",
          shadowOffsetY: 10,
          shoadowOffsetX: 10,
          shadowBlur: 10
        }
    },
      style: {          // default styling for the edge should come here
        lineWidth: 1, 
        opacity: 0.15,
        fill: '#fff',
        position: "middle",
        endArrow: { path: G6.Arrow.triangle(5, 10, 0), d: 0 },
      },
    },
    modes: {
      default: [
        "click-select",
        "drag-canvas",
        {
          type: "zoom-canvas",
          enableOptimize: true,
        },
      ],
    }
  };
</script>
<div class="m-0 h-screen bg-base-300">
  <div
    class="flex h-fit right-0 bottom-0 fixed align-bottom justify-center items-end"
    style="z-index:1"
  >
    <div class="block m-6">
      <ul class="menu w-16 py-3 shadow-lg bg-base-100 rounded-box">
        <li>
          <a
            class=""
            class:btn-active={interactiveMode}
            on:click={() => ( ( $interactiveMode=!$interactiveMode) )}
          >
          <svg class="svg-icon" viewBox="0 0 24 24" >
            <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="4"
            fill="#ffffff"
            d="M12.522,10.4l-3.559,3.562c-0.172,0.173-0.451,0.176-0.625,0c-0.173-0.173-0.173-0.451,0-0.624l3.248-3.25L8.161,6.662c-0.173-0.173-0.173-0.452,0-0.624c0.172-0.175,0.451-0.175,0.624,0l3.738,3.736C12.695,9.947,12.695,10.228,12.522,10.4 M18.406,10c0,4.644-3.764,8.406-8.406,8.406c-4.644,0-8.406-3.763-8.406-8.406S5.356,1.594,10,1.594C14.643,1.594,18.406,5.356,18.406,10M17.521,10c0-4.148-3.374-7.521-7.521-7.521c-4.148,0-7.521,3.374-7.521,7.521c0,4.147,3.374,7.521,7.521,7.521C14.147,17.521,17.521,14.147,17.521,10"></path>
          </svg>
          </a>
        </li>
        <li>
          <a class="" class:btn-active={show_menu} on:click={toggleMenu}>
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              class="inline-block w-6 h-6 stroke-current"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M21 9.5H3M21 4.5H3M21 14.5H3M21 19.5H3"
              />
            </svg>
          </a>
        </li>
        <li>
          <a class="" class:btn-active={show_legend} on:click={toggleLegend}>
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              class="inline-block w-6 h-6 stroke-current"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
              />
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"
              />
            </svg>
          </a>
        </li>
        <li>
          <a
            class=""
            class:btn-active={show_timebar}
            on:click={() => (
              (show_timebar = !show_timebar),
              graph_elem.updateTimeBar(show_timebar)
            )}
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              class="inline-block w-6 h-6 stroke-current"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M21.5 12H12V2.5"
              />
              <circle
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                cx="12"
                cy="12"
                r="10"
              />
            </svg>
          </a>
        </li>
      </ul>
    </div>
    <div
      class="p-3 shadow-lg bg-base-100 rounded-box h-fit w-fit mb-6"
      style="z-index:1"
      class:hidden={!show_menu}
    >
      <GraphOptions
        bind:graph_options
        on:reset={reset_graph_options}
        on:apply={consume_query}
      />
    </div>
    <div
      style="z-index:1"
      class="overflow-x-auto overflow-y-auto bg-base-100 w-0 h-fit shadow-lg rounded-box mb-6"
      class:show={show_legend}
    >
      <ColorLegend {styles} />
    </div>
  </div>
  <div
    style="z-index:1"
    class="p-3 shadow-lg bg-base-100 rounded-box h-fit left-0 bottom-0 fixed w-fit m-6"
  >
    <div class="container h-full w-full p-1 overflow-hidden scroll-auto">
      <div class:hidden={!selected_node} class="rounded-box bg-accent p-3 mb-2">
        <EventDetail {selected_node} on:useroot={use_selected_as_root} />
      </div>
      <h1 class="text-lg py-2">Filter Options:</h1>
      <QueryForm bind:query={current_query} />
      <div class="btn-group w-full flex flex-row mt-2">
        <button class="btn btn-sm btn-primary basis-1/3" on:click={add_filter}>
          + new filter</button
        >
        <button
          class="btn btn-sm btn-primary basis-1/3"
          disabled={qhistory.length <= 1 || awaiting_query_request}
          on:click={() => {
            qhistory.pop();
            current_query = qhistory.pop();
            qhistory = [...qhistory];
            submit_state_query();
          }}
          >{qhistory.length - 1 > 0
            ? "undo " + (qhistory.length - 1)
            : ":)"}</button
        >
        <button
          class="btn btn-sm btn-primary basis-1/3"
          class:loading={awaiting_query_request}
          disabled={awaiting_query_request || !current_query_changed}
          on:click={submit_state_query}>submit</button
        >
      </div>
    </div>
  </div>
  <div class="w-fit h-10
            right-3
             top-10
             fixed
             mr-10
             mb-6            
             "
             class:hidden={!show_message}
             class:show= {show_message}
             >
    <span class="text-sm text-left w-full h-full">LATEST EVENTS RECEIVED - {dayToDisplay} AT {displayTime}</span> 
  </div>
  <G6Graph
    on:nodeselected={on_node_selected}
    bind:this={graph_elem}
    {options}
    data={{}}
  />
</div>
<style lang="postcss" global>
  @tailwind base;
  @tailwind components;
  @tailwind utilities;
  .show {
    width: 320px;
  }
  .move {
    margin-right: 350px;
  }
  input::-webkit-outer-spin-button,
  input::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }
  /* Firefox */
  input[type="number"] {
    -moz-appearance: textfield;
  }
</style>