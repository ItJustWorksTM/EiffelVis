<script lang="ts">
    export let values = [];
    const filter_types: string[] = ["ID", "Type", "Source", "Host", "Tag"];
</script>

<div class="form-control">
    <div class="flex flex-row px-1.5" />
    <table>
        <thead>
            <tr>
                <th> Field </th>
                <th> Value </th>
                <th> Exclude </th>
                <th> Active </th>
                <th> Wildcard </th>
            </tr>
        </thead>
        {#each values as value, i}
            <tr>
                <td>
                    <select
                        class="select select-primary max-w-xs"
                        bind:value={value.filterField}
                    >
                        {#each filter_types as type}
                            <option>{type}</option>
                        {/each}
                    </select>
                </td>
                <td>
                    <input
                        type="text"
                        placeholder={value.filterField}
                        bind:value={value.value}
                        class="input input-sm input-bordered flex-1 w-40"
                    />
                </td>
                <td
                    ><input
                        type="checkbox"
                        class="toggle toggle-md toggle-primary flex-none "
                        bind:checked={value.exclude}
                    /></td
                >
                <td>
                    <input
                        type="checkbox"
                        class="toggle toggle-md toggle-primary flex-none "
                        bind:checked={value.active}
                    /></td
                >
                <td>
                    <input
                        type="checkbox"
                        class="toggle toggle-md toggle-primary flex-none "
                        bind:checked={value.isWildCard}
                    /></td
                >
                <td>
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
                    <slot index={i} />
                </td>
            </tr>
        {/each}
    </table>
</div>
