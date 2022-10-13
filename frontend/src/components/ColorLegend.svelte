<script lang="ts">
    export let colors;
</script>

<table class="table w-full h-fit">
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
