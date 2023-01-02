<script lang="ts">
   import SvgButton from "./SvgButton.svelte";

   export let show_menu: boolean;
   export let show_legend: boolean;
   export let show_timebar: boolean;

   export let updateTimeBarPlaceholder: () => void; //it was called directly inside the div element, now: a method is created inside app.svelt
   export let toggleMenuPlaceholder: () => void;
   export let toggleLegendPlaceholder: () => void;
</script>

<div class="flex h-screen w-16">
   <ul
      class="menu grid justify-items-center content-end py-3  h-screen shadow-large bg-base-300"
   >
      <li>
         <SvgButton
            onClickAction={toggleMenuPlaceholder}
            btnState={show_menu}
            data="M21 9.5H3M21 4.5H3M21 14.5H3M21 19.5H3"
         />
      </li>
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
            onClickAction={updateTimeBarPlaceholder}
            btnState={show_timebar}
            data="M21.5 12H12V2.5 M2 12 a 10,10 0 1 1 20 0 a 10 10 0 1 1 -20 0"
         />
      </li>
   </ul>
</div>
