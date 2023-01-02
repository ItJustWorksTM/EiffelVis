<script lang=ts>
    import Shortcut from "./Shortcut.svelte";

    /**
     * List of shortcuts. 
     * The label is the description of the shortcut
     * key_name is the name of the key associated to the shortcut -> displayed to the user
     * event_which is the number of the key emited in a key listener when pressed (easier when rebinding)
     * more info here: https://www.freecodecamp.org/news/javascript-keycode-list-keypress-event-key-codes/
     */
    let shortcuts = [
        { label: "Toggle timebar", key: "t", event_which: 84},
        { label: "Hide all panels", key: "h", event_which: 72},
        { label: "Toggle filter", key: "f", event_which: 70},
        { label: "Toggle settings", key: "s", event_which: 83},
        { label: "Toggle legend", key: "l", event_which: 76},
        { label: "Toggle non interactive mode", key: "n", event_which: 78},
        { label: "Navigate graph left", key: "◀︎", event_which: 37 },
        { label: "Navigate graph right", key: "▶︎", event_which: 39 },
        { label: "Navigate graph up", key: "▲", event_which: 38 },
        { label: "Navigate graph down", key: "▼", event_which: 40 },
        { label: "Increase speed while navigating (maintain)", key: "⇪", event_which: 16 },
    ]

</script>

<div class="p-3 shortcut_container flex flex-col max-height-full" >
    {#each shortcuts as shortcut}
        <Shortcut shortcut_label={shortcut.label} shortcut_key={shortcut.key.toUpperCase()}/>
    {/each}
</div>
