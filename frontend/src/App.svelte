<!-- svelte-ignore a11y-missing-attribute -->
<script lang="ts">
    import G6 from '@antv/g6';
    import { QueryStream, EiffelVisConnection } from './eiffelvis';
    import { GraphSettings, StatefulLayout } from './layout';
    import G6Graph from './components/G6Graph.svelte';
    import SideBar from './components/SideBar.svelte';
    import Panel from './components/Panel.svelte';
    import { FullEvent, query_eq } from './apidefinition';
    import { deep_copy } from './utils';
    import config from './config.json';
    import { empty_fixed_event_filters, FixedQuery, fixed_query_to_norm } from './uitypes';
    import Settings from './components/settings/Settings.svelte';

    export let connection: EiffelVisConnection;

    /* Boolean to handle when a text field is focused to ignore keyboard shortcuts */
    let isTextFieldFocused: boolean = false;

    let graph_elem: G6Graph = null;
    let active_stream: QueryStream = null;
    let awaiting_query_request: boolean = false;

    let selected_node: FullEvent = null;

    let show_settings: boolean = false;
    let show_legend: boolean = true;
    let show_timebar: boolean = false;
    let show_filter_panel: boolean = false;
    $: nonInteractiveState = true;

    let customTheme: Object = config.Theme.ColorBlind;
    let themeMap: Map<string, any> = new Map(Object.entries(customTheme));
    let legend: Map<string, any> = themeMap;
    $: styles = [...legend.entries()];

    let query_cache: { stream: QueryStream; query: FixedQuery }[] = [];

    let qhistory: FixedQuery[] = [];

    let appKeyMap: Object = {};

    let current_query: FixedQuery = {
        range_filter: { begin: { type: 'Absolute', val: -500 }, end: null },
        event_filters: [empty_fixed_event_filters()],
        collection: { type: 'Forward' },
    };

    $: current_query_changed =
        qhistory.length > 0 &&
        !query_eq(
            fixed_query_to_norm(current_query),
            fixed_query_to_norm(qhistory[qhistory.length - 1]),
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
    let show_message: boolean = false;
    let dayToDisplay: string = null;
    let dayLastEventRecieved: number = 0;
    let recievedNewNode: boolean = false;
    let displayTime: string = null;
    let displayDate: string = null;

    const displayInfoMessage = () => {
        //After 1 minute of no nodes recieved, a message is displayed.
        let time: Date = new Date();
        if (time.getDate() == dayLastEventRecieved) {
            dayToDisplay = 'TODAY';
        } else if (time.getDate() - dayLastEventRecieved == 1) {
            dayToDisplay = 'YESTERDAY';
        } else if (time.getDate() - dayLastEventRecieved > 1) {
            dayToDisplay = displayDate;
        }

        if (recievedNewNode == false && dayToDisplay != null) {
            show_message = true;
            nonInteractiveState = false;
            console.log('received no new node');
        } else {
            show_message = false;
        }
    };

    let ms = 60000;
    let interval = setInterval(displayInfoMessage, ms); // set timer to run every 1 minute

    // timer function to wait 1 minute to check if nodes are still being received,
    // if no new nodes after 1 minute, message for latest node received is displayed
    const resetTimer = () => {
        clearInterval(interval); // interval is reset every minute
        interval = setInterval(displayInfoMessage, ms);
    };

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

            graph_elem.nonInteractiveMode(event, nonInteractiveState);

            //every time a node is pushed to the graph the variables are updated
            let timeJson: number = event.time;
            let time: Date = new Date(timeJson);
            dayLastEventRecieved = time.getDate();
            displayDate = time.toLocaleDateString([], {
                weekday: 'short',
                day: 'numeric',
                month: 'short',
                year: 'numeric',
            });
            displayTime = time.toLocaleTimeString([], {
                hour: '2-digit',
                minute: '2-digit',
            });
            recievedNewNode = true;
            show_message = false;

            // TODO: Find a better way to do this
            if (once) {
                graph_elem.focusNode(event.id);
                once = false;
            }

            legend = layout.getNodeStyle();
        }

        recievedNewNode = false;
        console.log('stoped recieving nodes');
        resetTimer(); // method to reset timer
    };

    const submit_state_query = () => submit_query(current_query);

    const submit_query = (fquery: FixedQuery) => {
        const new_query = fixed_query_to_norm(fquery);
        active_stream = (() => {
            const cached = query_cache.find(v => query_eq(new_query, fixed_query_to_norm(v.query)));
            if (cached) {
                return cached.stream;
            } else {
                const ret = new QueryStream(connection, deep_copy(new_query));
                query_cache = [...query_cache, { stream: ret, query: deep_copy(fquery) }];
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
            selected_node = await connection.fetch_node(e.detail.target._cfg.model.id);
        } else {
            selected_node = null;
        }
    };

    const use_selected_as_root = () => {
        current_query.collection = { type: 'AsRoots' };
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

    const toggleSettings = () => {
        show_settings = !show_settings;
    };

    const toggleLegend = () => {
        show_legend = !show_legend;
    };

    const toggleFilterPanel = () => {
        show_filter_panel = !show_filter_panel;
    };

    //Updates the timebar each time the show timebar button is clicked
    const updateTimebar = () => {
        (show_timebar = !show_timebar), graph_elem.updateTimeBar(show_timebar);
    };

    const toggleInteractiveMode = () => {
        nonInteractiveState = !nonInteractiveState;
    };

    const options = {
        width: 400,
        height: 400,
        workerEnabled: false,
        fitView: true,
        fitViewPadding: [0, 0, 0, 600],
        groupByTypes: false, // enables to control z-index of items https://antv-g6.gitee.io/en/docs/manual/middle/elements/methods/elementIndex
        defaultEdge: {
            labelCfg: {
                position: 'center',
                style: {
                    // default styling for the edge labels should come here https://g6.antv.vision/en/docs/manual/middle/elements/edges/defaultEdge
                    fontSize: 10,
                    fill: '#ffffff',
                    fillOpacity: 0,
                    shadowColor: '#151517',
                    shadowOffsetY: 10,
                    shoadowOffsetX: 10,
                    shadowBlur: 10,
                },
            },
            style: {
                // default styling for the edge should come here
                lineWidth: 1,
                opacity: 0.3,
                fill: '#fff',
                position: 'middle',
                endArrow: { path: G6.Arrow.triangle(5, 10, 0), d: 0 },
            },
        },
        modes: {
            default: [
                'click-select',
                'drag-canvas',
                {
                    type: 'zoom-canvas',
                    enableOptimize: true,
                },
            ],
        },
    };

    const handle_close_request = () => {
        console.log('received in app');
        show_settings = !show_settings;
    };

    const handleKeyDown = (e: KeyboardEvent): void => {
        if (isTextFieldFocused) return;

        appKeyMap[e.key] = e.type == 'keydown';

        if (
            (appKeyMap['ArrowRight'] ||
                appKeyMap['ArrowLeft'] ||
                appKeyMap['ArrowUp'] ||
                appKeyMap['ArrowDown']) &&
            nonInteractiveState
        ) {
            nonInteractiveState = false;
        }

        // Hide all panels
        if (appKeyMap['h'] || appKeyMap['H']) {
            if (show_legend) toggleLegend();

            if (show_settings) toggleSettings();

            if (show_filter_panel) toggleFilterPanel();

            if (show_timebar) updateTimebar();
        }

        // Legend panel
        if (appKeyMap['l'] || appKeyMap['L']) {
            toggleLegend();
        }

        // Options panel
        if (appKeyMap['s'] || appKeyMap['S']) {
            toggleSettings();
        }

        // Non-interactive mode
        if (appKeyMap['n'] || appKeyMap['N']) {
            toggleInteractiveMode();
        }

        // Timebar
        if (appKeyMap['t'] || appKeyMap['T']) {
            updateTimebar();
        }

        // Filter panel
        if (appKeyMap['f'] || appKeyMap['F']) {
            toggleFilterPanel();
        }
    };

    const handleKeyUp = (e: KeyboardEvent): void => {
        appKeyMap[e.key] = e.type == 'keydown';
    };
</script>

<svelte:window on:keydown={handleKeyDown} on:keyup={handleKeyUp} />

<div class="flex w-screen h-screen relative bg-base-100">
    <!-- SideBar component: the variables are updated inside App.svelte -->
    <div class="z-20">
        <SideBar
            {show_timebar}
            {show_legend}
            {show_settings}
            interactiveMode={nonInteractiveState}
            {show_filter_panel}
            toggleSettingsPlaceholder={toggleSettings}
            toggleLegendPlaceholder={toggleLegend}
            toggleFilterPanelPlaceholder={toggleFilterPanel}
            updateTimeBarPlaceholder={updateTimebar}
            toggleInteractiveModePlaceholder={toggleInteractiveMode}
        />
    </div>
    <div class="flex z-10 pointer-events-none">
        <!-- panels  -->
        <Panel
            {show_filter_panel}
            show_legend_placeholder={show_legend}
            {use_selected_as_root}
            {current_query}
            {current_query_changed}
            {add_filter}
            {qhistory}
            {awaiting_query_request}
            submit_state_query_placeholder={submit_state_query}
            {selected_node}
            {styles}
            on:isBlur={() => {
                isTextFieldFocused = false;
            }}
            on:isFocused={() => {
                isTextFieldFocused = true;
            }}
        />
    </div>
    <div class="flex flex-col fixed z-0 items-center">
        <div style="white-space: nowrap;" class:hidden={!show_message} class:show={show_message}>
            <span class="text-sm text-left w-full h-full"
                >LATEST EVENTS RECEIVED - {dayToDisplay} AT {displayTime}</span
            >
        </div>
        <G6Graph
            on:nodeselected={on_node_selected}
            bind:this={graph_elem}
            bind:nonInteractiveState
            {options}
            data={{}}
            bind:isFocused={isTextFieldFocused}
        />
    </div>
    <div
        class="flex flex-wrap content-center justify-center z-30 absolute w-screen h-screen pointer-events-none rounded-lg"
    >
        <div
            class="pointer-events-auto rounded-lg w-3/6 min-w-min h-3/6 relative"
            class:hidden={!show_settings}
        >
            <Settings
                on:close_request={() => {
                    show_settings = !show_settings;
                }}
                {consume_query}
                reset_graph_options_placeholder={reset_graph_options}
                {graph_options}
            />
        </div>
    </div>
</div>

<style lang="postcss" global>
    @tailwind base;
    @tailwind components;
    @tailwind utilities;
    .show {
        width: 375px;
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
    input[type='number'] {
        -moz-appearance: textfield;
    }
</style>
