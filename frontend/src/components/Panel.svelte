<script lang="ts">
  import GraphOptions from "./GraphOptions.svelte";
  import ColorLegend from "./ColorLegend.svelte";
  import EventDetail from "./EventDetail.svelte";
  import QueryForm from "./QueryForm.svelte";
  import type { GraphSettings } from "../layout";
  import {
    empty_fixed_event_filters,
    FixedQuery,
    fixed_query_to_norm,
  } from "../uitypes";
  import { FullEvent, query_eq } from "../apidefinition";

  //Boolean variables
  export let show_legend_placeholder: boolean;
  export let show_menu_placeholder: boolean;
  export let awaiting_query_request: boolean;
  export let current_query_changed: boolean;

  let widget;

  $: current_query_changed =
    qhistory.length > 0 &&
    !query_eq(
      fixed_query_to_norm(current_query),
      fixed_query_to_norm(qhistory[qhistory.length - 1])
    );

  //Object variables; used for onClick actions.
  export let reset_graph_options_placeholder: () => void;
  export let consume_query: () => void;
  export let use_selected_as_root: () => void;
  export let add_filter: () => void;
  export let submit_state_query_placeholder: () => void;
  export let current_query: FixedQuery;
  export let qhistory: FixedQuery[];
  export let selected_node: FullEvent;
  export let graph_options: GraphSettings;
  export let styles: Object;
</script>

<div class="grid fixed bottom-0 bg-base-1" style="z-index:1">
  <ul class="menu menu-compact">
    <li>
      <div
        class="overflow-x-auto overflow-y-auto top-0 shadow-md fixed bg-base-200 w-0 h-fit rounded-r-lg"
        class:show={show_legend_placeholder}
      >
        <ColorLegend {styles} />
      </div>
    </li>
    <li>
      <div
        class="overflow-x-auto overflow-y-auto fixed top-0 shadow-md bg-base-200 w-0 h-fit mb-0 rounded-r-lg"
        class:show={show_menu_placeholder}
      >
        <GraphOptions
          bind:graph_options
          on:reset={reset_graph_options_placeholder}
          on:apply={consume_query}
        />
      </div>
    </li>
    <li>
      <div
        class="p-3 bg-base-200 shadow-md h-fit bottom-0 fixed w-fit m-0 rounded-r-lg"
      >
        <div class="container h-full w-full p-1 overflow-hidden scroll-auto">
          <div
            class:hidden={!selected_node}
            class="rounded-box bg-accent p-3 mb-2"
          >
            <EventDetail {selected_node} on:useroot={use_selected_as_root} />
          </div>
          <h1 class="text-lg py-2">Filter Options:</h1>
          <QueryForm bind:query={current_query} filterWidget={widget} />
          <div class="btn-group w-full flex flex-row mt-2">
            <button
              class="btn btn-sm btn-primary basis-1/3"
              on:click={add_filter}
            >
              + new filter</button
            >
            <button
              class="btn btn-sm btn-primary basis-1/3"
              disabled={qhistory.length <= 1 || awaiting_query_request}
              on:click={() => {
                qhistory.pop();
                current_query = qhistory.pop();
                qhistory = [...qhistory];
                submit_state_query_placeholder();
              }}
              >{qhistory.length - 1 > 0
                ? "undo " + (qhistory.length - 1)
                : ":)"}</button
            >
            <button
              class="btn btn-sm btn-primary basis-1/3"
              class:loading={awaiting_query_request}
              disabled={awaiting_query_request || !current_query_changed}
              on:click={() => {
                empty_fixed_event_filters;
                widget.add_tempfilter_to_query();
                // submit_state_query_placeholder;
              }}>submit</button
            >
          </div>
        </div>
      </div>
    </li>
  </ul>
</div>
