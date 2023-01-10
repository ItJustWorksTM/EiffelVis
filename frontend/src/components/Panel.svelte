<script lang="ts">
    import GraphOptions from './GraphOptions.svelte';
    import ColorLegend from './ColorLegend.svelte';
    import EventDetail from './EventDetail.svelte';
    import QueryForm from './QueryForm.svelte';
    import { FixedQuery, fixed_query_to_norm } from '../uitypes';
    import { FullEvent, query_eq } from '../apidefinition';

    //Boolean variables
    export let show_legend_placeholder: boolean;
    export let awaiting_query_request: boolean;
    export let current_query_changed: boolean;
    export let show_filter_panel: boolean;

    $: current_query_changed =
        qhistory.length > 0 &&
        !query_eq(
            fixed_query_to_norm(current_query),
            fixed_query_to_norm(qhistory[qhistory.length - 1]),
        );

    //Object variables; used for onClick actions.
    export let use_selected_as_root: () => void;
    export let add_filter: () => void;
    export let submit_state_query_placeholder: () => void;
    export let current_query: FixedQuery;
    export let qhistory: FixedQuery[];
    export let selected_node: FullEvent;
    export let styles: Object;
</script>

<div class="h-screen w-fit flex flex-col pointer-events-none bg-base-1">
    <div
        class="mb-auto pointer-events-auto shadow-md bg-base-200 rounded-r-lg"
        class:show={show_legend_placeholder}
        class:hidden={!show_legend_placeholder}
    >
        <ColorLegend {styles} />
    </div>
    <div
        class="mt-auto pointer-events-auto bg-base-200 shadow-md rounded-r-lg overflow-y-auto"
        class:show={show_filter_panel}
        class:hidden={!show_filter_panel}
    >
        <div class="container h-full w-full p-3 scroll-auto">
            <div class:hidden={!selected_node} class="rounded-box bg-accent p-3 mb-2">
                <EventDetail {selected_node} on:useroot={use_selected_as_root} />
            </div>
            <h1 class="text-lg py-2">Filter Options</h1>
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
                        submit_state_query_placeholder();
                    }}>{qhistory.length - 1 > 0 ? 'undo ' + (qhistory.length - 1) : ':)'}</button
                >
                <button
                    class="btn btn-sm btn-primary basis-1/3"
                    class:loading={awaiting_query_request}
                    disabled={awaiting_query_request || !current_query_changed}
                    on:click={submit_state_query_placeholder}>submit</button
                >
            </div>
        </div>
    </div>
</div>
