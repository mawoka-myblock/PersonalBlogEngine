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
        tags: string[]
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
            }
        }
    }
</script>
<button class="fixed top-0 left-0 bg-green-500" on:click={() => {postSelected = null}}>Close</button>
<button class="fixed top-0 right-0 bg-green-500" on:click={savePost}>Save</button>
<div class="pt-6">
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
            </div>
            <svelte:component this={c.default} bind:value={post.content}/>
        {/await}

    {/await}
</div>