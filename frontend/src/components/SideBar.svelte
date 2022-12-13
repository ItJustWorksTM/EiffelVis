<script lang="ts">
  import SvgButton from "./SvgButton.svelte";

   export let show_menu: boolean;
   export let show_legend: boolean;
   export let show_timebar: boolean;
   export let interactiveMode: boolean;

   export let updateTimeBarPlaceholder: () => void; //it was called directly inside the div element, now: a method is created inside app.svelt
   export let toggleMenuPlaceholder: () => void;  
   export let toggleLegendPlaceholder: () => void;
   export let toggleInteractiveModePlaceholder: () => void;

</script>
<div class="flex h-screen w-16">
    <ul class="menu grid justify-items-center content-end py-3  h-screen shadow-large bg-base-300">
      <li> 
        <SvgButton onClickAction={toggleInteractiveModePlaceholder} 
                btnState= {interactiveMode} 
                data= "M12.522,10.4l-3.559,3.562c-0.172,0.173-0.451,0.176-0.625,0c-0.173-0.173-0.173-0.451,0-0.624l3.248-3.25L8.161,6.662c-0.173-0.173-0.173-0.452,0-0.624c0.172-0.175,0.451-0.175,0.624,0l3.738,3.736C12.695,9.947,12.695,10.228,12.522,10.4 M18.406,10c0,4.644-3.764,8.406-8.406,8.406c-4.644,0-8.406-3.763-8.406-8.406S5.356,1.594,10,1.594C14.643,1.594,18.406,5.356,18.406,10M17.521,10c0-4.148-3.374-7.521-7.521-7.521c-4.148,0-7.521,3.374-7.521,7.521c0,4.147,3.374,7.521,7.521,7.521C14.147,17.521,17.521,14.147,17.521,10"   
        />
      </li>
      <li>
        <SvgButton
            onClickAction= {toggleMenuPlaceholder} 
            btnState= {show_menu}  
            data = "M21 9.5H3M21 4.5H3M21 14.5H3M21 19.5H3" 
         />
      </li>
      <li>
        <SvgButton 
            onClickAction= {toggleLegendPlaceholder} 
            btnState= {show_legend} 
            data = "M15 12a3 3 0 11-6 0 3 3 0 016 0z M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268
                     2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"
        />
      </li>
      <li> 
        <SvgButton onClickAction={updateTimeBarPlaceholder} 
                btnState= {show_timebar} 
                data= "M21.5 12H12V2.5 M2 12 a 10,10 0 1 1 20 0 a 10 10 0 1 1 -20 0"   
        />
      </li>
    </ul>
  </div>
