<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    const dispatch: Function = createEventDispatcher();

    /* Title of the component the header is going to be attached to */
    export let title: String; 

</script>

<div class="flex bg-base-200 p-3 items-center">
    <p class="mr-auto text-4xl">{title}</p>
    <!-- https://daisyui.com/components/button/ -> circle button -->
    <button class="btn btn-circle ml-auto" on:click={() => {dispatch('close_request')}}>
        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /></svg>
    </button>
</div>