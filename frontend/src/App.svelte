<!-- svelte-ignore a11y-missing-attribute -->
<script lang="ts">
	import G6 from '@antv/g6';
	import { QueryStream, EiffelVisConnection } from './eiffelvis';
	import { StatefulLayout } from './layout';
	import FilterWidget from './components/FilterWidget.svelte';
	import type { GraphSettings, Query } from './apidefinition';
	import { deep_copy } from './utils';
	import G6Graph from './components/G6Graph.svelte';
	import { toggle_class } from 'svelte/internal';

	let graph_elem: G6Graph | null;

	const backend_url = process.env.EIFFELVIS_URL.startsWith('@origin')
		? `${window.location.host}${
				window.location.pathname
		  }${process.env.EIFFELVIS_URL.replace('@origin', '')}`
		: process.env.EIFFELVIS_URL.startsWith('@hostname')
		? `${window.location.hostname}${process.env.EIFFELVIS_URL.replace(
				'@hostname',
				''
		  )}`
		: process.env.EIFFELVIS_URL;
	const backend_has_ssl = JSON.parse(process.env.EIFFELVIS_SSL);
	const backend_proto_ws = backend_has_ssl ? 'wss' : 'ws';
	const backend_proto_http = backend_has_ssl ? 'https' : 'http';

	const conn = new EiffelVisConnection(
		`${backend_proto_ws}://${backend_url}/ws`
	);
	let stream = null;

	let selected_node = null;

	let waiting = false;

	let show_menu = false;

	let show_legend = false;

	let legend = new Map<string, string>();

	let colors = new Array<[string, string]>();

	// TODO: make a real type
	const newDefault = () => {
		return {
			ids: {
				rev: false,
				pred: { type: 'Id', ids: [] },
			},
			tags: {
				rev: false,
				pred: { type: 'Tag', tags: [] },
			},
			types: {
				rev: false,
				pred: { type: 'Type', names: [] },
			},
			sourcehosts: {
				rev: false,
				pred: { type: 'SourceHost', hosts: [] },
			},
			sourcenames: {
				rev: false,
				pred: { type: 'SourceName', names: [] },
			},
		};
	};

	let qhistory = [];

	let filters = [newDefault()] as any;

	let collection_modes = ['Forward', 'AsRoots'];
	let collection_mode = 'Forward';

	const range_modes = ['Time', 'Absolute', 'Ids'];
	let begin_mode = 'Absolute';
	let begin_value = '-500';

	let end_mode = 'Time';
	let end_value = '';

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
			graph_elem.resizeGraph();
			selected_node = null;
			submitCurrentQuery();
		}
	}

	const consumeQuery = async () => {
		const layout = new StatefulLayout();
		waiting = true;
		const iter = await stream.iter();
		waiting = false;
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
			legend = layout.getNodeColor();
		}
	};

	$: {
		colors = [...legend.entries()];
	}

	const submitCurrentQuery = () => {
		const query: Query = {
			range_filter: {
				begin:
					begin_value.length > 0
						? ({
								type: begin_mode as any,
								val: begin_mode == 'Ids' ? begin_value : parseInt(begin_value),
						  } as any)
						: null,
				end:
					end_value.length > 0
						? ({
								type: end_mode as any,
								val: end_mode === 'Ids' ? end_value : parseInt(end_value),
						  } as any)
						: null,
			},
			event_filters: filters
				.map((fil: any) => {
					const ret = [];

					const push_if = (arr: any[], obj: any) => {
						if (arr.length > 0) ret.push(deep_copy(obj));
					};

					push_if(fil.ids.pred.ids, fil.ids);
					push_if(fil.tags.pred.tags, fil.tags);
					push_if(fil.types.pred.names, fil.types);
					push_if(fil.sourcehosts.pred.hosts, fil.sourcehosts);
					push_if(fil.sourcenames.pred.names, fil.sourcenames);
					return ret as any;
				})
				.filter((fil: any[]) => fil.length > 0),
			collection: { type: collection_mode as 'Forward' | 'AsRoots' },
		};

		qhistory = [
			...qhistory,
			{
				collection_mode: collection_mode,
				filters: deep_copy(filters),
				begin_mode: `${begin_mode}`,
				end_mode: `${end_mode}`,
				begin_value: `${begin_value}`,
				end_value: `${end_value}`,
			},
		];

		const newq = new QueryStream(conn, query);
		stream = newq;
		consumeQuery();
	};

	const onNodeSelected = async (e: any) => {
		if (e.detail?.target) {
			selected_node = await fetch(
				`${backend_proto_http}://${backend_url}/get_event/${e.detail.target._cfg.model.id}`
			).then(resp => resp.json());
		} else {
			selected_node = null;
		}
	};

	const resetGraphOptions = () => {
		graph_options = {
			offset: 0,
			time_diff: 1000,
			y_scale: 0.99,
			x_sep: 60,
			y_sep: 60,
			hue: 360,
		};
		consumeQuery();
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
		defaultEdge: {
			style: {
				endArrow: { path: G6.Arrow.triangle(5, 10, 0), d: 0 },
			},
		},
		nodeStateStyles: {
			selected: {
				fill: '#ffffff',
				lineWidth: 0.4,
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
</script>

<main class="m-0 h-screen bg-base-300">
	<div
		id="sideBar"
		class="h-full w-0 fixed z-1 top-0 right-0 bg-base-100 overflow-x-hidden pt-0"
		class:open={show_menu}
	>
		<div
			class=" border border-base-300 rounded-box p-6 pt-5 overflow-y-auto bg-base-200 flex-col shadow-lg bg-base-100"
		>
			<h1 class="text-lg py-2">Graph Options</h1>
			<label class="input-group input-group-sm mt-1">
				<span
					class="border border-[rgba(255,255,255,0.2)] span w-1/2 bg-base-100"
					>Offset</span
				>
				<input
					type="number"
					bind:value={graph_options.offset}
					class="input input-bordered input-sm w-1/2"
				/>
			</label>
			<label class="input-group input-group-sm mt-1">
				<span
					class="span border border-[rgba(255,255,255,0.2)] w-1/2 bg-base-100"
					>Time Collapse</span
				>
				<input
					type="number"
					bind:value={graph_options.time_diff}
					class="input input-bordered input-sm w-1/2"
				/>
			</label>
			<label class="input-group input-group-sm mt-1">
				<span
					class="span border border-[rgba(255,255,255,0.2)] w-1/2 bg-base-100"
					>Y-axis Scaling</span
				>
				<input
					type="number"
					bind:value={graph_options.y_scale}
					class="input input-bordered input-sm w-1/2"
				/>
			</label>
			<label class="input-group input-group-sm mt-1">
				<span
					class="span border border-[rgba(255,255,255,0.2)] w-1/2 bg-base-100"
					>X-axis Node Separation</span
				>
				<input
					type="number"
					bind:value={graph_options.x_sep}
					class="input input-bordered input-sm w-1/2"
				/>
			</label>
			<label class="input-group input-group-sm mt-1">
				<span
					class="span border border-[rgba(255,255,255,0.2)] w-1/2 bg-base-100"
					>Y-axis Node Separation</span
				>
				<input
					type="number"
					bind:value={graph_options.y_sep}
					class="input input-bordered input-sm w-1/2"
				/>
			</label>
			<label class="input-group input-group-sm mt-1">
				<span
					class="span border border-[rgba(255,255,255,0.2)] w-1/2 bg-base-100"
					>Hue (Node Color)</span
				>
				<input
					type="number"
					bind:value={graph_options.hue}
					class="input input-bordered input-sm w-1/2"
				/>
			</label>
			<div class="btn-group w-full flex flex-row mt-2">
				<button class="btn btn-sm grow btn-primary" on:click={consumeQuery}
					>Update Graph</button
				>
				<button class="btn btn-sm grow btn-primary" on:click={resetGraphOptions}
					>Reset Default</button
				>
			</div>
		</div>
	</div>

	<div
		class="right-6 bottom-0 mb-6 inline-block absolute"
		class:move={show_legend || show_menu}
	>
		<ul class="menu w-16 py-3 shadow-lg bg-base-100 rounded-box">
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
		</ul>
	</div>

	<div
		class="overflow-x-auto overflow-y-auto bg-base-100 w-0 h-100 absolute rounded-box right-0 bottom-0 m-6"
		class:show={show_legend}
	>
		<table class="table w-full">
			<thead>
				<tr>
					<th>Event Type</th>
					<th>Color</th>
				</tr>
			</thead>
			<tbody>
				{#each colors as [event, color]}
					<tr class="">
						<td>{event}</td>
						<td
							>&nbsp
							<div class="avatar h-3">
								<div
									class="mb-8 rounded-full w-5 h-5"
									style="background-color: {color}"
								/>
							</div></td
						>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>

	<div
		class="p-3 shadow-lg bg-base-100 rounded-box h-fit left-0 bottom-0 fixed w-fit m-6"
	>
		<div class="container h-full w-full p-1 overflow-hidden scroll-auto">
			<h1 class="text-lg py-2">Filter Options:</h1>
			<div class:hidden={!selected_node} class="rounded-box bg-accent p-3 mb-2">
				<p>Time: {selected_node?.meta.time}</p>
				<p>Type: {selected_node?.meta.type}</p>
				<p>Host: {selected_node?.meta.source.host}</p>
				<p>Source: {selected_node?.meta.source.name}</p>
				<p>
					Tags: {selected_node?.meta.tags ? selected_node?.meta.tags : 'n/a'}
				</p>

				<!-- <a class="font-mono">{selected_node?.meta.id}</a> -->
				<div
					data-tip="Use this node as the graph root"
					class="tooltip tooltip-neutral w-full"
				>
					<button
						class="btn btn-xs btn-accent font-mono w-full select-all"
						on:click={async () => {
							const nw = newDefault();
							nw.ids.pred.ids = [selected_node.meta.id];
							collection_mode = 'AsRoots';
							filters = [nw];
							begin_value = '';
							end_value = '';
							submitCurrentQuery();
						}}>{selected_node?.meta.id}</button
					>
				</div>
			</div>

			<!-- TODO: Support range filter -->
			{#each filters as filter, i}
				<div
					tabindex="0"
					class="grow collapse w-full border rounded-box border-base-300 collapse-arrow"
				>
					<input type="checkbox" />
					<div class="collapse-title text-base font-medium">
						{`Filter ${i}`}
					</div>
					<div class="collapse-content">
						<FilterWidget
							bind:ids={filter.ids}
							bind:tags={filter.tags}
							bind:types={filter.types}
							bind:sourcehosts={filter.sourcehosts}
							bind:sourcenames={filter.sourcenames}
						/>
					</div>
				</div>
			{/each}
			<div class="form-control">
				<!-- svelte-ignore a11y-label-has-associated-control -->
				<label class="input-group input-group-sm mt-1">
					<select
						class="select select-bordered select-sm"
						bind:value={begin_mode}
					>
						{#each range_modes as mode}
							<option>{mode}</option>
						{/each}
					</select>
					<input
						type="text"
						bind:value={begin_value}
						placeholder={'Begin'}
						class="input input-bordered input-sm w-full"
					/>
				</label>
				<label class="input-group input-group-sm mt-1">
					<select
						class="select select-bordered select-sm"
						bind:value={end_mode}
					>
						{#each range_modes as mode}
							<option>{mode}</option>
						{/each}
					</select>
					<input
						type="text"
						bind:value={end_value}
						placeholder={'End'}
						class="input input-bordered input-sm w-full"
					/>
				</label>
			</div>
			<div class="btn-group w-full flex flex-row mt-2">
				{#each collection_modes as mode}
					<button
						class="btn btn-xs grow"
						class:btn-active={mode == collection_mode}
						on:click={() => (collection_mode = mode)}>{mode}</button
					>
				{/each}
			</div>
			<div class="btn-group w-full flex flex-row mt-2">
				<button
					class="btn btn-sm btn-primary basis-1/3"
					on:click={() => (filters = [...filters, newDefault()])}
				>
					+ new filter</button
				>
				<button
					class="btn btn-sm btn-primary btn-outline basis-1/3"
					class:btn-disabled={qhistory.length <= 1 || waiting}
					on:click={() => {
						qhistory.pop();
						qhistory = [...qhistory];
						const q = qhistory.pop();
						collection_mode = q.collection_mode;
						filters = q.filters;
						begin_mode = q.begin_mode;
						begin_value = q.begin_value;
						end_mode = q.end_mode;
						end_value = q.end_value;

						submitCurrentQuery();
					}}
					>{qhistory.length - 1 > 0
						? 'undo ' + (qhistory.length - 1)
						: ':)'}</button
				>
				<button
					class="btn btn-sm btn-primary basis-1/3"
					class:loading={waiting}
					class:btn-disabled={waiting}
					on:click={submitCurrentQuery}>submit</button
				>
			</div>
		</div>
	</div>

	<G6Graph
		on:nodeselected={e => onNodeSelected(e)}
		bind:this={graph_elem}
		{options}
		data={{}}
	/>
</main>

<style lang="postcss" global>
	@tailwind base;
	@tailwind components;
	@tailwind utilities;
	.open {
		width: 330px;
	}
	.show {
		width: 300px;
	}
	.move {
		margin-right: 320px;
	}
</style>
