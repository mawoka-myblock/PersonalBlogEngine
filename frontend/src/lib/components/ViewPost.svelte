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
        tags: string[],
        thumbs_up?: number,
        thumbs_down?: number,
    }

    let post: Post | null = null;
    const postSchema = y.object({
        intro: y.string().required(),
        title: y.string().required(),
        content: y.string().required(),
        tags: y.array().of(y.string()).required(),
        published: y.boolean()
    })


    const fetchPost = async (): Promise<Post | null> => {
        const res = await fetch(`/api/v1/manage/post?slug=${postSelected}`);
        if (res.status === 200) {
            const json = await res.json();
            post = json
            return json
        } else {
            post = {
                slug: postSelected,
                title: "",
                content: "",
                published: false,
                tags: ["test"],
                intro: "",
            }
            return null;

        }
    }
    const savePost = async (): Promise<void> => {
        if (!(await postSchema.isValid(post))) {
            return;
        }
        const res = await fetch("/api/v1/manage/update", {
            method: "PUT",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(post)
        })
        const resBody = await res.text();
        if (res.status !== 200) {
            if (resBody === "NotFound") {
                const resp = await fetch("/api/v1/manage/create_post", {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json"
                    },
                    body: JSON.stringify(post)
                })
                if (resp.status !== 200) {
                    alert("Couldn't save post!")
                    return
                }
            } else {
                alert("Couldn't save post!")
                return

            }
        }
    }
</script>
<button
        type="button"
        class="fixed top-0 left-0 bg-white rounded-md p-2 inline-flex items-center justify-center text-gray-400 hover:text-gray-500 hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-indigo-500 place-self-end"
        on:click={() => {
            savePost().then();
			postSelected = null;
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
<button
        type="button"
        class="fixed top-0 right-0 bg-white rounded-md p-2 inline-flex items-center justify-center text-gray-400 hover:text-gray-500 hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-indigo-500 place-self-end"
        on:click={savePost}
>
    <span class="sr-only">Save Post</span>
    <!-- Heroicons: outline/save -->
    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
              d="M8 7H5a2 2 0 00-2 2v9a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-3m-1 4l-3 3m0 0l-3-3m3 3V4"></path>
    </svg>
</button>
<div class="pt-12">
    {#await fetchPost()}
        <Spinner/>

    {:then p}
        {#await import("./Editor.svelte")}
            <Spinner/>
        {:then c}
            <div class="flex flex-col">
                <label>Public? <input type="checkbox" bind:checked={post.published}></label>
                <label>Title <input type="text" bind:value={post.title}
                                    class="text-black w-full rounded-lg p-1"
                                    class:border-red-600={!y.reach(postSchema, 'title').isValidSync(post.title)}
                                    class:border-solid={!y.reach(postSchema, 'title').isValidSync(post.title)}
                                    class:border-2={!y.reach(postSchema, 'title').isValidSync(post.title)}
                ></label>
                <label>Intro <textarea bind:value={post.intro}
                                       class="text-black w-full rounded-lg p-1"
                                       class:border-red-600={!y.reach(postSchema, 'intro').isValidSync(post.intro)}
                                       class:border-solid={!y.reach(postSchema, 'intro').isValidSync(post.intro)}
                                       class:border-2={!y.reach(postSchema, 'intro').isValidSync(post.intro)}></textarea></label>
                <div>
                    <!-- heroicons/thumb-up -->
                    <svg class="inline-block h-8 w-8 align-middle rounded-full bg-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"
                         xmlns="http://www.w3.org/2000/svg">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                              d="M14 10h4.764a2 2 0 011.789 2.894l-3.5 7A2 2 0 0115.263 21h-4.017c-.163 0-.326-.02-.485-.06L7 20m7-10V5a2 2 0 00-2-2h-.095c-.5 0-.905.405-.905.905 0 .714-.211 1.412-.608 2.006L7 11v9m7-10h-2M7 20H5a2 2 0 01-2-2v-6a2 2 0 012-2h2.5"></path>
                    </svg>
                    <span class="align-middle">{post.thumbs_up || "0"}</span>
                </div>
                <div>
                    <!-- heroicons/thumb-down -->
                    <svg class="inline-block h-8 w-8 align-middle rounded-full bg-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"
                         xmlns="http://www.w3.org/2000/svg">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                              d="M14 10h4.764a2 2 0 011.789 2.894l-3.5 7A2 2 0 0115.263 21h-4.017c-.163 0-.326-.02-.485-.06L7 20m7-10V5a2 2 0 00-2-2h-.095c-.5 0-.905.405-.905.905 0 .714-.211 1.412-.608 2.006L7 11v9m7-10h-2M7 20H5a2 2 0 01-2-2v-6a2 2 0 012-2h2.5"></path>
                    </svg>
                    <span class="align-middle">{post.thumbs_down || "0"}</span>
                </div>
            </div>
            <svelte:component this={c.default} bind:value={post.content}/>
        {/await}

    {/await}
</div>