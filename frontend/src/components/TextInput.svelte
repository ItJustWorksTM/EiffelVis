<script lang="ts">
    export let value: string;
    export let placeholder: string;
</script>

<input
    type="text"
    placeholder={placeholder}
    bind:value={value}
    class="input input-sm input-bordered basis-full"
/>
