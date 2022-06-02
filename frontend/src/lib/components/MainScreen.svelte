<script lang="ts">
    import Spinner from "./Spinner.svelte"

    interface ListPostResponse {
        slug: string
        title: string
        created_at: string
        updated_at: string
        tags: string[]
        intro: string
        published: boolean
    }

    let postSelected: null | string = null

    const loadAllPosts = async (): Promise<Array<ListPostResponse | null>> => {
        const res = await fetch("/api/v1/manage/posts?offset=0")
        return await res.json()
    }

    const createNewPost = () => {
        postSelected = prompt("Enter slug for new post")
    }
</script>

{#if postSelected === null}
    {#await loadAllPosts()}
        <Spinner/>

    {:then data}
        {#if data.length === 0}
            <div class="pt-12 grid place-items-center">
                <button class="bg-white rounded-md p-2 inline-flex items-center justify-center text-gray-400 hover:text-gray-500 hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-indigo-500 text-xl"
                on:click={createNewPost}>
                    <svg class="w-12 h-12" fill="none" stroke="currentColor" viewBox="0 0 24 24"
                         xmlns="http://www.w3.org/2000/svg">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                              d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
                    </svg>
                    <span class="text-black">Create a new Post</span>
                </button>
            </div>
        {:else}
            <button on:click={createNewPost}>Create new Post</button>
            <div class="flex flex-col">
                <div class="overflow-x-auto sm:-mx-6 lg:-mx-8">
                    <div class="py-2 inline-block min-w-full sm:px-6 lg:px-8">
                        <div class="overflow-hidden">
                            <table class="min-w-full">
                                <thead class="border-b">
                                <tr>
                                    <th scope="col" class="text-sm font-medium text-gray-900 px-6 py-4 text-left">
                                        Slug
                                    </th>
                                    <th scope="col" class="text-sm font-medium text-gray-900 px-6 py-4 text-left">
                                        Title
                                    </th>
                                    <th scope="col" class="text-sm font-medium text-gray-900 px-6 py-4 text-left">
                                        Published
                                    </th>
                                    <th scope="col" class="text-sm font-medium text-gray-900 px-6 py-4 text-left">
                                        Created At
                                    </th>
                                </tr>
                                </thead>
                                <tbody>
                                {#each data as post}
                                    <tr class="bg-white border-b">
                                        <td class="text-sm text-gray-900 font-light px-6 py-4 whitespace-nowrap">
                                            <button on:click={() => {postSelected= post.slug}}>
                                                {post.slug}
                                            </button>
                                        </td>
                                        <td class="text-sm text-gray-900 font-light px-6 py-4 whitespace-nowrap">
                                            {post.title}
                                        </td>
                                        <td class="text-sm text-gray-900 font-light px-6 py-4 whitespace-nowrap">
                                            {#if post.published}
                                                ✅
                                            {:else}
                                                ❌
                                            {/if}
                                        </td>
                                        <td class="text-sm text-gray-900 font-light px-6 py-4 whitespace-nowrap">
                                            {post.created_at}
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
{:else}
    {#await import("./ViewPost.svelte") then c}
        <svelte:component this={c.default} bind:postSelected/>
    {/await}
{/if}