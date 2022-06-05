<script lang="ts">
    import "bytemd/dist/index.css";
    import { Editor, Viewer } from "bytemd";
    import gfm from "@bytemd/plugin-gfm";

    export let value;
    const plugins = [gfm()];

    function handleChange(e) {
        value = e.detail.value;
    }
</script>

<div class="prose w-screen">
    <div class="w-screen">
        <Editor {plugins} {value} on:change={handleChange} />
    </div>
</div>
