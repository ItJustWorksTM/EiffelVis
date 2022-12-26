<script lang="ts">
  import HeaderWithCloseButton from "../utils/HeaderWithCloseButton.svelte";
  import GraphOptions from "../GraphOptions.svelte";
  import type { GraphSettings } from "../../layout";
  import ShortcutPanel from "../shortcut/ShortcutPanel.svelte";

  /*  Graph options elements */
  export let graph_options: GraphSettings;
  export let consume_query: () => void;
  export let reset_graph_options_placeholder: () => void;

  /* Props for the Header component */
  const header_title = "Settings";

  /* List of the different settings to be displayed. 
  * Adding an object here will create a new button automatically
  * The selected boolean is used to control style and hidden status of HTML component the button is supposed to control */
  let setting_list = [
    {
      name: "Graph Options",
      selected: true,
    },
    {
      name: "Shortcuts",
      selected: false,
    },
  ];
  
  /* Method to hide all containers in the setting_list.
  Necessary to display only 1 setting option at the time. */
  const hide_all_settings_choices = () => {
    for(const setting_choice of setting_list){
      setting_choice.selected=false
    }
  };

</script>

<div class="flex flex-col h-full w-full point-events-auto bg-base-200">
  <!-- Header of the setting component -->
  <HeaderWithCloseButton title={header_title} on:close_request/>
  <div class="flex overflow-y-auto h-full w-full justify-between">
    <!-- To add a new choice, add an object in the variable setting_list -->
    <div class="p-3 flex flex-col overflow-y-auto h-full w-1/3">
      <ul>
        {#each setting_list as setting_choice}
          <li>
            <!-- Buttons when clicked will unselect all choices before selecting the one clicked on -->
            <button
              class="btn btn-wide w-full"
              on:click={hide_all_settings_choices}
              on:click={() => {setting_choice.selected = !setting_choice.selected}}
              class:btn-active={setting_choice.selected}
              >
                {setting_choice.name}
            </button>
          </li>
        {/each}
      </ul>
    </div> 
    <div class="w-2/3 overflow-y-auto h-full">
      <!-- Use class:hidden{<selected value of the button controlling the visibility>} to give visibility control to the button -->
      <div
      class="bg-base-200 h-full"
      class:hidden={!setting_list[0].selected}
      >
      <GraphOptions
        bind:graph_options
        on:reset={reset_graph_options_placeholder}
        on:apply={consume_query}
      />
      </div> 
      <div>
        <ShortcutPanel />
      </div> 
    </div>
  </div>
</div>
