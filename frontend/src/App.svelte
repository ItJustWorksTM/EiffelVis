<!-- svelte-ignore a11y-missing-attribute -->
<script lang="ts">
	import G6 from "@antv/g6";
	import { QueryStream, EiffelVisConnection } from "./eiffelvis";
	import { StatefulLayout } from "./layout";
	import FilterWidget from "./components/FilterWidget.svelte";
	import type { Query } from "./apidefinition";
	import { deep_copy } from "./utils";
	import ThreeJSGraph from "./components/ThreeJSGraph.svelte";
	// import G6Graph from "./components/G6Graph.svelte";

	let graph_elem: ThreeJSGraph | null;

	const backendurl = process.env.EIFFELVIS_URL;

	const conn = new EiffelVisConnection(`ws://${backendurl}/ws`);
	let stream = null;

	let selected_node = null;

	let waiting = false;

	// TODO: make a real type
	const newDefault = () => {
		return {
			ids: {
				rev: false,
				pred: { type: "Id", ids: [] },
			},
			tags: {
				rev: false,
				pred: { type: "Tag", tags: [] },
			},
			types: {
				rev: false,
				pred: { type: "Type", names: [] },
			},
			sourcehosts: {
				rev: false,
				pred: { type: "SourceHost", hosts: [] },
			},
			sourcenames: {
				rev: false,
				pred: { type: "SourceName", names: [] },
			},
		};
	};

	let qhistory = [];

	let filters = [newDefault()] as any;

	let collection_modes = ["Forward", "AsRoots"];
	let collection_mode = "Forward";

	const range_modes = ["Time", "Absolute", "Ids"];
	let begin_mode = "Absolute";
	let begin_value = "-500";

	let end_mode = "Time";
	let end_value = "";

	$: {
		if (graph_elem) {
			// graph_elem.resizeGraph();
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
			layout.apply(event);
			graph_elem.push(event);

			// TODO: Find a better way to do this
			if (once) {
				// graph_elem.focusNode(event.id);
				once = false;
			}
		}
	};

	const submitCurrentQuery = () => {
		const query: Query = {
			range_filter: {
				begin:
					begin_value.length > 0
						? ({
								type: begin_mode as any,
								val:
									begin_mode == "Ids"
										? begin_value
										: parseInt(begin_value),
						  } as any)
						: null,
				end:
					end_value.length > 0
						? ({
								type: end_mode as any,
								val:
									end_mode === "Ids"
										? end_value
										: parseInt(end_value),
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
			collection: { type: collection_mode as "Forward" | "AsRoots" },
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
				`http://${backendurl}/get_event/${e.detail.target._cfg.model.id}`
			).then((resp) => resp.json());
		} else {
			selected_node = null;
		}
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
				fill: "#ffffff",
				lineWidth: 0.4,
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

<main class="m-0 h-screen bg-base-300">
	<div
		class="p-3 shadow-lg bg-base-100 rounded-box h-fit left-0 bottom-0 fixed w-fit m-6"
	>
		<div class="container h-full w-full p-1 overflow-hidden scroll-auto">
			<div
				class:hidden={!selected_node}
				class="rounded-box bg-accent p-3 mb-2"
			>
				<p>Time: {selected_node?.meta.time}</p>
				<p>Type: {selected_node?.meta.type}</p>
				<p>Host: {selected_node?.meta.source?.host}</p>
				<p>Source: {selected_node?.meta.source?.name}</p>
				<p>
					Tags: {selected_node?.meta.tags
						? selected_node?.meta.tags
						: "n/a"}
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
							collection_mode = "AsRoots";
							filters = [nw];
							begin_value = "";
							end_value = "";
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
						placeholder={"Begin"}
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
						placeholder={"End"}
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
						? "undo " + (qhistory.length - 1)
						: ":)"}</button
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

	<ThreeJSGraph bind:this={graph_elem} />
</main>

<style lang="postcss" global>
	@tailwind base;
	@tailwind components;
	@tailwind utilities;
</style>
