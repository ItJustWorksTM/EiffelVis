<script lang="ts">
    export let value: string;
    export let placeholder: string;
</script>

<input type="text" {placeholder} bind:value class="input input-sm input-bordered basis-full" />
