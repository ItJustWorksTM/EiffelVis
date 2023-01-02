<!-- svelte-ignore a11y-missing-attribute -->
<script lang="ts">
  import G6 from "@antv/g6";
  import { QueryStream, EiffelVisConnection } from "./eiffelvis";
  import { GraphSettings, StatefulLayout } from "./layout";
  import G6Graph from "./components/G6Graph.svelte";
  import SideBar from "./components/SideBar.svelte";
  import Panel from "./components/Panel.svelte";
  import { FullEvent, query_eq } from "./apidefinition";
  import { deep_copy } from "./utils";
  import config from "./config.json";

  import {
    empty_fixed_event_filters,
    FixedQuery,
    fixed_query_to_norm,
    TemperateFilterArray,
  } from "./uitypes";

  export let connection: EiffelVisConnection;
  let event_filters_sets: TemperateFilterArray[] = [[]];
  $: event_filters_sets;

  let graph_elem: G6Graph = null;
  let active_stream: QueryStream = null;
  let awaiting_query_request: boolean = false;

  let selected_node: FullEvent = null;

  let show_menu: boolean = false;
  let show_legend: boolean = true;
  let show_timebar: boolean = false;

  let customTheme: Object = config.Theme.ColorBlind;
  let themeMap: Map<string, any> = new Map(Object.entries(customTheme));
  let legend: Map<string, any> = themeMap;
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

      // TODO: Find a better way to do this
      if (once) {
        graph_elem.focusNode(event.id);
        once = false;
      }

      legend = layout.getNodeStyle();
    }
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

  const add_filter_set = () => {
    if (event_filters_sets) {
      let empty_filter_set = [];
      event_filters_sets.push(empty_filter_set);
    } else {
      event_filters_sets = [];
    }
    console.log(event_filters_sets);
  };

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
    filters.ids[0] = {
      rev: false,
      pred: { type: "Id", ids: [selected_node.meta.id] },
    };
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

  //Updates the timebar each time the show timebar button is clicked
  const updateTimebar = () => {
    (show_timebar = !show_timebar), graph_elem.updateTimeBar(show_timebar);
  };

  const options = {
    width: 400,
    height: 400,
    workerEnabled: false,
    fitView: true,
    groupByTypes: false, // enables to control z-index of items https://antv-g6.gitee.io/en/docs/manual/middle/elements/methods/elementIndex
    defaultEdge: {
      labelCfg: {
        position: "center",
        style: {
          // default styling for the edge labels should come here https://g6.antv.vision/en/docs/manual/middle/elements/edges/defaultEdge
          fontSize: 10,
          fill: "#ffffff",
          fillOpacity: 0,
          shadowColor: "#151517",
          shadowOffsetY: 10,
          shoadowOffsetX: 10,
          shadowBlur: 10,
        },
      },
      style: {
        // default styling for the edge should come here
        lineWidth: 1,
        opacity: 0.15,
        fill: "#fff",
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
    },
  };
</script>

<div class="fixed flex m-0 h-screen w-screen bg-base-100">
  <!-- SideBar component: the variables are updated inside App.svelte -->
  <SideBar
    {show_timebar}
    {show_legend}
    {show_menu}
    toggleMenuPlaceholder={toggleMenu}
    toggleLegendPlaceholder={toggleLegend}
    updateTimeBarPlaceholder={updateTimebar}
  />
  <div class="grid w-screen h-screens" style="z-index:1">
    <!-- panels  -->
    <Panel
      bind:event_filters_sets
      show_legend_placeholder={show_legend}
      show_menu_placeholder={show_menu}
      reset_graph_options_placeholder={reset_graph_options}
      {use_selected_as_root}
      bind:current_query
      bind:current_query_changed
      {add_filter_set}
      bind:qhistory
      bind:awaiting_query_request
      submit_state_query_placeholder={submit_state_query}
      {consume_query}
      {selected_node}
      {graph_options}
      {styles}
    />
    <!-- Graph with listeners -->
    <G6Graph
      on:nodeselected={on_node_selected}
      bind:this={graph_elem}
      {options}
      data={{}}
    />
  </div>
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
