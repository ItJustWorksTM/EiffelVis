<script lang="ts">
    export let btnState: boolean;
    export let onClickAction: () => void;
    export let data: string;
</script>

<!-- svelte-ignore a11y-missing-attribute -->
<a class:active={btnState} on:click={onClickAction}>
    <svg
        xmlns="http://www.w3.org/2000/svg"
        fill="none"
        viewBox="0 0 24 24"
        class="inline-block w-6 h-6 stroke-current"
    >
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d={data} />
    </svg>
</a>
