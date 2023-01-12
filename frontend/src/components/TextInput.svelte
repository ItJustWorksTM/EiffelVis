<script lang="ts">
    import { createEventDispatcher } from 'svelte';

    const dispatch: Function = createEventDispatcher();

    export let value: string;
    export let placeholder: string;

    const onFocus: VoidFunction = (): void => {
        dispatch('isFocused');
    };

    const onBlur: VoidFunction = (): void => {
        dispatch('isBlur');
    };
</script>

<input
    type="text"
    {placeholder}
    bind:value
    class="input input-sm input-bordered basis-full"
    on:focus={onFocus}
    on:blur={onBlur}
/>
