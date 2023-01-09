<script lang="ts">
    import { createEventDispatcher } from 'svelte';

    // Label attached to the input text. Desciption of the shortcut
    export let shortcut_label: string = 'Not defined';
    // Key to be displayed within the input text
    export let shortcut_key: string;

    const dispatch: Function = createEventDispatcher();

    // Dispatch an event when the value of the input has been changed. Useful for rebinding
    const change_event = (event: Event) => {
        dispatch('change', event);
    };
</script>

<div class="grid gap-3 grid-cols-4 items-center bg-base-100 my-1 rounded-lg">
    <label class="col-start-1 col-end-4 pl-3" for="shortcut">{shortcut_label}</label>
    <input
        on:change={change_event}
        bind:value={shortcut_key}
        placeholder="None"
        id="shortcut"
        type="text"
        maxlength="1"
        size="1"
        class="input bg-base-300 mr-3 my-1 text-center col-start-4 col-end-5"
        readonly
    />
</div>
