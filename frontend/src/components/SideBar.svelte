<script lang="ts">
   import SvgButton from "./SvgButton.svelte";
   import ButtonSeparator from "./Sidebar/ButtonSeparator.svelte";

   export let show_settings: boolean;
   export let show_legend: boolean;
   export let show_timebar: boolean;
   export let interactiveMode: boolean;
   export let show_filter_panel: boolean;

   export let updateTimeBarPlaceholder: () => void; //it was called directly inside the div element, now: a method is created inside app.svelt
   export let toggleSettingsPlaceholder: () => void;
   export let toggleLegendPlaceholder: () => void;
   export let toggleInteractiveModePlaceholder: () => void;
   export let toggleFilterPanelPlaceholder: () => void;
</script>

<div class="flex h-screen">
   <ul
      class="menu grid justify-items-center content-end py-3  h-screen shadow-large bg-base-300"
   >
      <!-- Buttons interracting with panels -->
      <li>
         <SvgButton
            onClickAction={toggleLegendPlaceholder}
            btnState={show_legend}
            data="M15 12a3 3 0 11-6 0 3 3 0 016 0z M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268
                      2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"
         />
      </li>

      <li>
         <SvgButton
            onClickAction={toggleFilterPanelPlaceholder}
            btnState={show_filter_panel}
            data="M3, 11 a 8,8 0 1,0 16, 0 a 8,8 0 1, 0 -16, 0  M 21 21 16.65 16.65"
         />
      </li>

      <ButtonSeparator />

      <li>
         <!-- TODO: ADD RIGHT SVG FOR SETTINGS -->
         <SvgButton
            onClickAction={toggleSettingsPlaceholder}
            btnState={show_settings}
            data="M9, 12 a 3,3 0 1,0 6, 0 a 3,3 0 1, 0 -6, 0 M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"
         />
      </li>

      <ButtonSeparator />

      <!-- Buttons interracting with graph -->
      <li>
         <SvgButton
            onClickAction={toggleInteractiveModePlaceholder}
            btnState={interactiveMode}
            data="M12.522,10.4l-3.559,3.562c-0.172,0.173-0.451,0.176-0.625,0c-0.173-0.173-0.173-0.451,0-0.624l3.248-3.25L8.161,6.662c-0.173-0.173-0.173-0.452,0-0.624c0.172-0.175,0.451-0.175,0.624,0l3.738,3.736C12.695,9.947,12.695,10.228,12.522,10.4 M18.406,10c0,4.644-3.764,8.406-8.406,8.406c-4.644,0-8.406-3.763-8.406-8.406S5.356,1.594,10,1.594C14.643,1.594,18.406,5.356,18.406,10M17.521,10c0-4.148-3.374-7.521-7.521-7.521c-4.148,0-7.521,3.374-7.521,7.521c0,4.147,3.374,7.521,7.521,7.521C14.147,17.521,17.521,14.147,17.521,10"
         />
      </li>
      <li>
         <SvgButton
            onClickAction={updateTimeBarPlaceholder}
            btnState={show_timebar}
            data="M21.5 12H12V2.5 M2 12 a 10,10 0 1 1 20 0 a 10 10 0 1 1 -20 0"
         />
      </li>
   </ul>
</div>
