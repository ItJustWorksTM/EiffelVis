<script lang="ts">
    export let values;
    export let placeholder;
    export let exclude;
    export let active;
</script>

<div class="form-control">
    <div class="flex flex-row px-1.5" />
    {#each values as value, i}
        <div class="flex flex-row px-1.5">
            <input
                type="text"
                {placeholder}
                bind:value
                class="input input-sm input-bordered flex-1 w-40"
            /><input
                type="checkbox"
                class="toggle toggle-md toggle-primary flex-none "
                bind:checked={exclude}
            />
            <input
                type="checkbox"
                class="toggle toggle-md toggle-primary flex-none "
                bind:checked={active}
            />
            <button
                class="btn btn-circle btn-xs my-auto mr-1"
                on:click={() => {
                    values.splice(i, 1);
                    values = [...values];
                }}
            >
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    class="inline-block w-4 h-4 stroke-current"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M6 18L18 6M6 6l12 12"
                    />
                </svg>
            </button>
        </div>
    {/each}
</div>
