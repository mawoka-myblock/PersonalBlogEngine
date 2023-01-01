<script lang="ts">
    import Spinner from "./Spinner.svelte";
    import {upload_files} from "./upload_files";

    let file_data: Array<{ id: string; date_added: string; mime_type: string; file_name: string }> | undefined = undefined

    let offset = 0

    let input_file_el: HTMLInputElement


    const get_file_data_fn = async () => {
        const res = await fetch(`/api/v1/uploads/list?offset=${offset}`)
        if (res.ok) {
            file_data = await res.json()
            return file_data
        }
    }

    const upload_file = async () => {
        const file = input_file_el.files[0]
        if (!file) {
            return
        }
        await upload_files(file)

    }

    const delete_file = async (file_id: string) => {
        if (!confirm("Do you really want to delete this file?")) {
            return
        }
        await fetch(`/api/v1/uploads/${file_id}`, {method: "DELETE"})
        get_file_data = get_file_data_fn()

    }
    let get_file_data = get_file_data_fn()

</script>

{#await get_file_data}
    <Spinner/>

{:then data}
    <div class="flex justify-center">
        <form on:submit|preventDefault={upload_file}>
            <input type="file" bind:this={input_file_el}>
            <button type="submit">Submit</button>
        </form>
    </div>

    {#if data.length !== 0}
        <div class="flex flex-col">
            <div class="overflow-x-auto sm:-mx-6 lg:-mx-8">
                <div class="py-2 inline-block min-w-full sm:px-6 lg:px-8">
                    <div class="overflow-hidden">
                        <table class="min-w-full">
                            <thead class="border-b">
                            <tr>
                                <th
                                        scope="col"
                                        class="text-sm font-medium text-gray-900 px-6 py-4 text-left"
                                >
                                    File Name
                                </th>
                                <th
                                        scope="col"
                                        class="text-sm font-medium text-gray-900 px-6 py-4 text-left"
                                >
                                    Created At
                                </th>
                                <th
                                        scope="col"
                                        class="text-sm font-medium text-gray-900 px-6 py-4 text-left"
                                >
                                    Delete
                                </th>
                                <th
                                        scope="col"
                                        class="text-sm font-medium text-gray-900 px-6 py-4 text-left"
                                >
                                    Download
                                </th>
                            </tr>
                            </thead>
                            <tbody>
                            {#each data as file}
                                <tr
                                        class="bg-white border-b"
                                >
                                    <td
                                            class="text-sm text-gray-900 font-light px-6 py-4 whitespace-nowrap"
                                    >
                                        {file.file_name}
                                    </td>
                                    <td
                                            class="text-sm text-gray-900 font-light px-6 py-4 whitespace-nowrap"
                                    >
                                        {file.date_added}
                                    </td>
                                    <td
                                            class="text-sm text-gray-900 font-light px-6 py-4 whitespace-nowrap"
                                    >
                                        <button on:click={() => {delete_file(file.id)}}>Delete</button>
                                    </td>
                                    <td
                                            class="text-sm text-gray-900 font-light px-6 py-4 whitespace-nowrap"
                                    >
                                        <a href="/api/v1/uploads/{file.id}" download>Download</a>
                                    </td>
                                </tr>
                            {/each}
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        </div>
    {/if}


{/await}