<script lang="ts">
    import Spinner from "./Spinner.svelte";
    import * as y from "yup";

    export let postSelected: null | string;

    interface Post {
        updated_at?: string;
        rendered_content?: string;
        intro: string;
        created_at?: string;
        published: boolean;
        title: string;
        slug: string;
        content: string;
        tags: string[];
        thumbs_up?: number;
        thumbs_down?: number;
    }

    enum SaveButtonStatus {
        Neutral,
        Loading,
        Success,
        Error,
    }

    let unsavedChanges = false;
    let originalPostSringified;

    let saveButtonStatus: SaveButtonStatus = SaveButtonStatus.Neutral;
    let post: Post | null = null;
    const postSchema = y.object({
        intro: y.string().required(),
        title: y.string().required(),
        content: y.string().required(),
        tags: y.array().of(y.string()).required(),
        published: y.boolean(),
    });
    let tempTagAdd = "";

    const fetchPost = async (): Promise<Post | null> => {
        const res = await fetch(`/api/v1/manage/post?slug=${postSelected}`);
        if (res.status === 200) {
            const json = await res.json();
            post = json;
            originalPostSringified = JSON.stringify(post);
            return json;
        } else {
            post = {
                slug: postSelected,
                title: "",
                content: "",
                published: false,
                tags: [],
                intro: "",
            };
            originalPostSringified = JSON.stringify(post);
            return null;
        }
    };
    const savePost = async (): Promise<void> => {
        saveButtonStatus = SaveButtonStatus.Loading;
        if (!(await postSchema.isValid(post))) {
            saveButtonStatus = SaveButtonStatus.Error;
            return;
        }
        const res = await fetch("/api/v1/manage/update", {
            method: "PUT",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(post),
        });
        originalPostSringified = await res.text();
        if (res.status !== 200) {
            const resp = await fetch("/api/v1/manage/create_post", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify(post),
            });
            if (resp.status !== 200) {
                console.log("Here");
                alert("Couldn't save post!");
                saveButtonStatus = SaveButtonStatus.Error;
                return;
            }
        } else {
            if (res.status !== 200) {
                console.log("thgere");
                alert("Couldn't save post!");
                saveButtonStatus = SaveButtonStatus.Error;
                return;
            }
        }
        saveButtonStatus = SaveButtonStatus.Success;
        unsavedChanges = false;
        setTimeout(() => {
            saveButtonStatus = SaveButtonStatus.Neutral;
        }, 1500);
    };
    const checkForUnsavedChanges = () => {
        try {
            originalPostSringified.updated_at = undefined;
            post.updated_at = undefined;
        } catch {}
        unsavedChanges = originalPostSringified !== JSON.stringify(post);
        // console.log(originalPostSringified, post);
    };
    $: {
        post;
        checkForUnsavedChanges();
    }
    const confirmUnload = () => {
        if (!unsavedChanges) {
            return;
        }
        event.preventDefault();
        event.returnValue = "";
    };
</script>

<svelte:window on:beforeunload={confirmUnload} />
<button
    type="button"
    class="fixed top-0 left-0 bg-white rounded-md p-2 inline-flex items-center justify-center text-gray-400 hover:text-gray-500 hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-indigo-500 place-self-end"
    on:click={() => {
        if (unsavedChanges) {
            if (
                confirm(
                    "There are unsaved changes. Do you really want to leave now?"
                )
            ) {
                postSelected = null;
            } else {
            }
        } else {
            postSelected = null;
        }
    }}
>
    <span class="sr-only">Close menu</span>
    <!-- Heroicons: outline/x -->
    <svg
        class="h-6 w-6"
        xmlns="http://www.w3.org/2000/svg"
        fill="none"
        viewBox="0 0 24 24"
        stroke="currentColor"
        aria-hidden="true"
    >
        <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M6 18L18 6M6 6l12 12"
        />
    </svg>
</button>
<span class="fixed top-0 right-0 inline-flex">
    <button
        type="button"
        class="rounded-md p-2 inline-flex items-center justify-center text-gray-400 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-indigo-500 place-self-end"
        class:bg-white={saveButtonStatus === SaveButtonStatus.Neutral}
        class:bg-red-600={saveButtonStatus === SaveButtonStatus.Error}
        class:bg-green-600={saveButtonStatus === SaveButtonStatus.Success}
        class:bg-yellow-500={saveButtonStatus === SaveButtonStatus.Loading}
        class:text-black={saveButtonStatus !== SaveButtonStatus.Neutral}
        on:click={savePost}
    >
        <span class="sr-only">Save Post</span>
        {#if saveButtonStatus === SaveButtonStatus.Neutral}
            <!-- Heroicons: outline/save -->
            <svg
                class="w-6 h-6"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
                xmlns="http://www.w3.org/2000/svg"
            >
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M8 7H5a2 2 0 00-2 2v9a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-3m-1 4l-3 3m0 0l-3-3m3 3V4"
                />
            </svg>
        {:else if saveButtonStatus === SaveButtonStatus.Loading}
            <svg class="w-6 h-6 animate-spin" viewBox="3 3 18 18">
                <path
                    class="fill-blue-800"
                    d="M12 5C8.13401 5 5 8.13401 5 12C5 15.866 8.13401 19 12 19C15.866 19 19 15.866 19 12C19 8.13401 15.866 5 12 5ZM3 12C3 7.02944 7.02944 3 12 3C16.9706 3 21 7.02944 21 12C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12Z"
                />
                <path
                    class="fill-blue-100"
                    d="M16.9497 7.05015C14.2161 4.31648 9.78392 4.31648 7.05025 7.05015C6.65973 7.44067 6.02656 7.44067 5.63604 7.05015C5.24551 6.65962 5.24551 6.02646 5.63604 5.63593C9.15076 2.12121 14.8492 2.12121 18.364 5.63593C18.7545 6.02646 18.7545 6.65962 18.364 7.05015C17.9734 7.44067 17.3403 7.44067 16.9497 7.05015Z"
                />
            </svg>
        {:else if saveButtonStatus === SaveButtonStatus.Success}
            <!-- heroicons/check -->
            <svg
                class="w-6 h-6"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
                xmlns="http://www.w3.org/2000/svg"
            >
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M5 13l4 4L19 7"
                />
            </svg>
        {:else if saveButtonStatus === SaveButtonStatus.Error}
            <!-- heroicons/exclamation-circle -->
            <svg
                class="w-6 h-6"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
                xmlns="http://www.w3.org/2000/svg"
            >
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                />
            </svg>
        {/if}
    </button>
    {#if unsavedChanges}
        <span class="top-0 absolute right-0 mt-1 mr-1 flex h-3 w-3">
            <span
                class="absolute inline-flex h-full w-full animate-ping rounded-full bg-sky-400 opacity-75"
            />
            <span
                class="relative inline-flex h-3 w-3 rounded-full bg-sky-500"
            />
        </span>
    {/if}
</span>
<div class="pt-12">
    {#await fetchPost()}
        <Spinner />
    {:then p}
        {#await import("./Editor.svelte")}
            <Spinner />
        {:then c}
            <div class="flex flex-col">
                <div class="p-2">
                    <label
                        >Public? <input
                            type="checkbox"
                            bind:checked={post.published}
                        /></label
                    >
                </div>
                <div class="p-2">
                    <label
                        >Title <input
                            type="text"
                            bind:value={post.title}
                            class="text-black w-full rounded-lg p-1 border-black border"
                            class:border-red-600={!y
                                .reach(postSchema, "title")
                                .isValidSync(post.title)}
                            class:border-solid={!y
                                .reach(postSchema, "title")
                                .isValidSync(post.title)}
                            class:border-2={!y
                                .reach(postSchema, "title")
                                .isValidSync(post.title)}
                        /></label
                    >
                </div>
                <div class="p-2">
                    <label
                        >Intro <textarea
                            bind:value={post.intro}
                            class="text-black w-full rounded-lg p-1 border border-black"
                            class:border-red-600={!y
                                .reach(postSchema, "intro")
                                .isValidSync(post.intro)}
                            class:border-solid={!y
                                .reach(postSchema, "intro")
                                .isValidSync(post.intro)}
                            class:border-2={!y
                                .reach(postSchema, "intro")
                                .isValidSync(post.intro)}
                        /></label
                    >
                </div>
                <div class="p-2">
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
                    <span class="align-middle">{post.thumbs_up || "0"}</span>
                </div>
                <div class="p-2">
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
                    <span class="align-middle">{post.thumbs_down || "0"}</span>
                </div>
                <div class="p-2">
                    <label for="addTag">Add tag:</label>
                    <input
                        id="addTag"
                        type="text"
                        bind:value={tempTagAdd}
                        class="border-b border-dotted border-black"
                    />
                    <button
                        on:click={() => {
                            if (tempTagAdd === "") {
                                return;
                            }
                            post.tags = [...post.tags, tempTagAdd];
                            tempTagAdd = "";
                        }}
                    >
                        <span class="w-fit">
                            <!-- Heroicons: outline/plus -->
                            <svg
                                class="w-6 h-6 inline-flex items-center"
                                fill="none"
                                stroke="currentColor"
                                viewBox="0 0 24 24"
                                xmlns="http://www.w3.org/2000/svg"
                                ><path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M12 6v6m0 0v6m0-6h6m-6 0H6"
                                /></svg
                            >
                        </span>
                    </button>
                    <div class="flex flex-row gap-2">
                        {#each post.tags as tag, index}
                            <div class="flex justify-center w-fit">
                                <div
                                    class="mt-2 mr-1 inline-flex items-center rounded bg-indigo-100 text-sm"
                                >
                                    <span
                                        class="ml-2 mr-1 max-w-xs truncate leading-relaxed"
                                        >{tag}</span
                                    >
                                    <button
                                        on:click={() => {
                                            post.tags.splice(index);
                                            post.tags = post.tags;
                                        }}
                                        class="inline-block h-8 w-6 align-middle text-gray-500 hover:text-gray-600 focus:outline-none"
                                    >
                                        <!-- Heroicons: outline/x -->

                                        <svg
                                            class="mx-auto h-6 w-6 fill-current"
                                            fill="none"
                                            stroke="currentColor"
                                            viewBox="0 0 24 24"
                                            xmlns="http://www.w3.org/2000/svg"
                                        >
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                stroke-width="2"
                                                d="M6 18L18 6M6 6l12 12"
                                            />
                                        </svg>
                                    </button>
                                </div>
                            </div>
                        {/each}
                    </div>
                </div>
            </div>
            <svelte:component this={c.default} bind:value={post.content} />
        {/await}
    {/await}
</div>
