<script lang="ts">
    import Spinner from "./Spinner.svelte";
    import Tabs from "./Tabs.svelte";
    import FileTab from "./Files.svelte";

    interface ListPostResponse {
        slug: string;
        title: string;
        created_at: string;
        updated_at: string;
        tags: string[];
        intro: string;
        published: boolean;
    }

    let postSelected: null | string = null;

    let feedbackPopUp = null;

    const loadAllPosts = async (): Promise<Array<ListPostResponse | null>> => {
        const res = await fetch("/api/v1/manage/posts?offset=0");
        return await res.json();
    };

    const loadRecentFeedbackFunction = async (): Promise<
        Array<object | null>
    > => {
        const res = await fetch("/api/v1/feedback/?limit=10");
        return await res.json();
    };
    let recentFeedback = loadRecentFeedbackFunction();

    const createNewPost = () => {
        postSelected = prompt("Enter slug for new post");
    };
    let tabSelected = "Posts";
</script>

{#if postSelected === null}
    <Tabs
        available_tabs={["Posts", "Feedback", "Files"]}
        bind:selected_tab={tabSelected}
    />
    {#if tabSelected === "Posts"}
        {#await loadAllPosts()}
            <Spinner />
        {:then data}
            {#if data.length === 0}
                <div class="pt-12 grid place-items-center">
                    <button
                        class="bg-white rounded-md p-2 inline-flex items-center justify-center text-gray-400 hover:text-gray-500 hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-indigo-500 text-xl"
                        on:click={createNewPost}
                    >
                        <svg
                            class="w-12 h-12"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M12 6v6m0 0v6m0-6h6m-6 0H6"
                            />
                        </svg>
                        <span class="text-black">Create a new Post</span>
                    </button>
                </div>
            {:else}
                <button
                    class="bg-white rounded-md p-2 inline-flex items-center justify-center text-gray-400 hover:text-gray-500 hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-indigo-500 text-lg"
                    on:click={createNewPost}
                >
                    <svg
                        class="w-8 h-8"
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M12 6v6m0 0v6m0-6h6m-6 0H6"
                        />
                    </svg>
                    <span class="text-black">Create a new Post</span>
                </button>
                <div class="flex flex-col">
                    <div class="overflow-x-auto sm:-mx-6 lg:-mx-8">
                        <div
                            class="py-2 inline-block min-w-full sm:px-6 lg:px-8"
                        >
                            <div class="overflow-hidden">
                                <table class="min-w-full">
                                    <thead class="border-b">
                                        <tr>
                                            <th
                                                scope="col"
                                                class="text-sm font-medium text-gray-900 px-6 py-4 text-left"
                                            >
                                                Slug
                                            </th>
                                            <th
                                                scope="col"
                                                class="text-sm font-medium text-gray-900 px-6 py-4 text-left"
                                            >
                                                Title
                                            </th>
                                            <th
                                                scope="col"
                                                class="text-sm font-medium text-gray-900 px-6 py-4 text-left"
                                            >
                                                Published
                                            </th>
                                            <th
                                                scope="col"
                                                class="text-sm font-medium text-gray-900 px-6 py-4 text-left"
                                            >
                                                Created At
                                            </th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {#each data as post}
                                            <tr
                                                class="bg-white border-b cursor-pointer"
                                                on:click={() => {
                                                    postSelected = post.slug;
                                                }}
                                            >
                                                <td
                                                    class="text-sm text-gray-900 font-light px-6 py-4 whitespace-nowrap"
                                                >
                                                    {post.slug}
                                                </td>
                                                <td
                                                    class="text-sm text-gray-900 font-light px-6 py-4 whitespace-nowrap"
                                                >
                                                    {post.title}
                                                </td>
                                                <td
                                                    class="text-sm text-gray-900 font-light px-6 py-4 whitespace-nowrap"
                                                >
                                                    {#if post.published}
                                                        ✅
                                                    {:else}
                                                        ❌
                                                    {/if}
                                                </td>
                                                <td
                                                    class="text-sm text-gray-900 font-light px-6 py-4 whitespace-nowrap"
                                                >
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
    {:else if tabSelected === "Feedback"}
        {#await recentFeedback}
            <Spinner />
        {:then data}
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
                                            Rating
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
                                            Post
                                        </th>
                                        <th
                                            scope="col"
                                            class="text-sm font-medium text-gray-900 px-6 py-4 text-left"
                                        >
                                            Feedback-Text?
                                        </th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {#each data as feedback}
                                        <tr
                                            class="bg-white border-b cursor-pointer"
                                            on:click={() => {
                                                feedbackPopUp = feedback;
                                            }}
                                        >
                                            <td
                                                class="text-sm text-gray-900 font-light px-6 py-4 whitespace-nowrap"
                                            >
                                                {#if feedback.thumbs_up}
                                                    <!-- heroicons/thumb-up -->
                                                    <svg
                                                        class="inline-block h-8 w-8 align-middle rounded-full bg-green-600"
                                                        fill="none"
                                                        stroke="currentColor"
                                                        viewBox="0 0 24 24"
                                                        xmlns="http://www.w3.org/2000/svg"
                                                    >
                                                        <path
                                                            stroke-linecap="round"
                                                            stroke-linejoin="round"
                                                            stroke-width="2"
                                                            d="M14 10h4.764a2 2 0 011.789 2.894l-3.5 7A2 2 0 0115.263 21h-4.017c-.163 0-.326-.02-.485-.06L7 20m7-10V5a2 2 0 00-2-2h-.095c-.5 0-.905.405-.905.905 0 .714-.211 1.412-.608 2.006L7 11v9m7-10h-2M7 20H5a2 2 0 01-2-2v-6a2 2 0 012-2h2.5"
                                                        />
                                                    </svg>
                                                {:else}
                                                    <!-- heroicons/thumb-down -->
                                                    <svg
                                                        class="inline-block h-8 w-8 align-middle rounded-full bg-red-600"
                                                        fill="none"
                                                        stroke="currentColor"
                                                        viewBox="0 0 24 24"
                                                        xmlns="http://www.w3.org/2000/svg"
                                                    >
                                                        <path
                                                            stroke-linecap="round"
                                                            stroke-linejoin="round"
                                                            stroke-width="2"
                                                            d="M10 14H5.236a2 2 0 01-1.789-2.894l3.5-7A2 2 0 018.736 3h4.018a2 2 0 01.485.06l3.76.94m-7 10v5a2 2 0 002 2h.096c.5 0 .905-.405.905-.904 0-.715.211-1.413.608-2.008L17 13V4m-7 10h2m5-10h2a2 2 0 012 2v6a2 2 0 01-2 2h-2.5"
                                                        />
                                                    </svg>
                                                {/if}
                                            </td>
                                            <td
                                                class="text-sm text-gray-900 font-light px-6 py-4 whitespace-nowrap"
                                            >
                                                {feedback.created_at}
                                            </td>
                                            <td
                                                class="text-sm text-gray-900 font-light px-6 py-4 whitespace-nowrap"
                                            >
                                                {feedback.post.title}
                                            </td>
                                            <td
                                                class="text-sm text-gray-900 font-light px-6 py-4 whitespace-nowrap"
                                            >
                                                {#if feedback.feedback_text === null}
                                                    ❌
                                                {:else}
                                                    ✅
                                                {/if}
                                            </td>
                                        </tr>
                                    {/each}
                                </tbody>
                            </table>
                        </div>
                    </div>
                </div>
            </div>
        {/await}
        {:else if tabSelected === "Files"}
        <FileTab />
    {/if}
{:else}
    {#await import("./ViewPost.svelte") then c}
        <svelte:component this={c.default} bind:postSelected />
    {/await}
{/if}

{#if feedbackPopUp !== null}
    <div class="fixed top-0 left-0 w-screen h-screen z-10 bg-white">
        <button
            on:click={() => {
                feedbackPopUp = null;
            }}>Close</button
        >
        <p>Text:</p>
        <p>{feedbackPopUp.feedback_text}</p>
    </div>
{/if}
