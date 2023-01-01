<script lang="ts">
    import "bytemd/dist/index.css";
    import {Editor, Viewer} from "bytemd";
    import gfm from "@bytemd/plugin-gfm";
    import {upload_files} from "./upload_files";

    export let value;
    const plugins = [gfm()];

    function handleChange(e) {
        value = e.detail.value;
    }


    const upload_image = async (files: Array<File>) => {
        let promises = []
        for (const file of files) {
            promises.push(upload_files(file))
        }
        const res = await Promise.all(promises)
        console.log(res)
        return res
    }
</script>

<div class="prose w-screen">
    <div class="w-screen">
        <Editor {plugins} {value} on:change={handleChange} uploadImages={upload_image}/>
    </div>
</div>
