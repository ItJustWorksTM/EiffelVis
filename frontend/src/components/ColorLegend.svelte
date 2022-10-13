<script lang="ts">
    export let styles;
    import G6, { Graph } from "@antv/g6";
    import { onMount } from "svelte";
    import G6Component from "svelte-g6";
    const options = {
        container: "mountNode",
        width: 280,
        height: 200,
        workerEnabled: false,
    };

    let data = {};
    let counter = 0;
    onMount(() => {
        let array = [];

        styles.forEach((event) => {
            let node = {
                id: event[0],
                size: 10,
                type: event[1].Shape,
                style: {
                    fill: event[1].Color,
                },
                label: event[1].Acronym,
                labelCfg: {
                    style: {
                        fill: event[1].Color,
                    },
                    position: "right",
                },
            };
            array.push(node);
        });
        data = { nodes: array };
    });
</script>

<table class="table w-full h-fit">
    <G6Component {G6} {options} {data} />
</table>
