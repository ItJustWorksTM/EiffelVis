<script lang="ts">
<<<<<<< HEAD
    export let values = [];
    const filter_types: string[] = ["ID", "Type", "Source", "Host", "Tag"];
=======

    export let values;
>>>>>>> bfe06c7 (backend: allow client to somewhat control string matching)
</script>

<div class="form-control">
    <div class="flex flex-row px-1.5" />
    <table>
        <thead>
            <tr>
                <th align="center"> Field </th>
                <th align="center" colspan="2"> Value </th>
                <th />
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
                <td colspan="2">
                    <input
                        type="text"
                        placeholder={value.filterField}
                        bind:value={value.value}
                        class="input input-sm input-bordered flex-1 w-40"
                    />
<<<<<<< HEAD
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
=======
                </svg>
            </button>
            <slot index={i}></slot>
        </div>
    {/each}
>>>>>>> bfe06c7 (backend: allow client to somewhat control string matching)
</div>
