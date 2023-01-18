<script lang="ts">
    import { FilterType, TemperateFilterArray } from '../uitypes';
    export let values: TemperateFilterArray;
    const filter_types: string[] = ['ID', 'Type', 'Source', 'Host', 'Tag'];
</script>

<div class="form-control">
    <table>
        <thead>
            <tr>
                <th class="w-4/13" align="center"> Field </th>
                <th class="w-8/13" align="center" colspan="2"> Value </th>
                <th class="w-1/13"><div class="w-4" /></th>
            </tr>
        </thead>
        {#each values as value, i}
            <tr>
                <td>
                    <select class="select select-primary" bind:value={value.filterField}>
                        {#each filter_types as type}
                            <option>{type}</option>
                        {/each}
                    </select>
                </td>
                <td colspan="2">
                    <input
                        type="text"
                        placeholder={value.filterField}
                        bind:value={value.value}
                        class="input input-sm input-bordered basis-2/3 h-8"
                    />
                </td>
            </tr>
            <tr>
                <td align="center"
                    ><p>Exclude</p>
                    <input
                        type="checkbox"
                        class="toggle toggle-md toggle-primary flex-none "
                        bind:checked={value.exclude}
                    /></td
                >
                <td align="center">
                    <p>Active</p>
                    <input
                        type="checkbox"
                        class="toggle toggle-md toggle-primary flex-none "
                        bind:checked={value.active}
                    />
                </td>
                <td align="center">
                    <p>Substring</p>
                    <input
                        type="checkbox"
                        class="toggle toggle-md toggle-primary flex-none "
                        disabled={value.filterField == FilterType.ID}
                        bind:checked={value.isWildCard}
                    />
                </td>
                <td align="center">
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
            <tr>
                <div class="divider-vertical" />
            </tr>
        {/each}
    </table>
</div>
