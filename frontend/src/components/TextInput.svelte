<script lang="ts" context="module">
    export let value: string;
    export let placeholder: string;
    export let isFocused: boolean = false;
    const onFocus = () => (isFocused = true);
    const onBlur = () => (isFocused = false);
</script>

<input
    type="text"
    {placeholder}
    bind:value
    class="input input-sm input-bordered basis-full"
    on:focus={onFocus}
    on:blur={onBlur}
/>
