import {isError, PBE} from ".";

const post_slug = "test";
describe("pbe", () => {
    const pbe = new PBE("http://127.0.0.1:8080");
    test("Get Single Markdown Post", async () => {
        const res = await pbe.get_post(post_slug);
        if (!isError(res)) {
            expect(res.metadata.slug).toBe("test");
        } else {
            throw new Error("Failed");
        }
    });
    test("Get Single Markdown Post Not Found", async () => {
        const res = await pbe.get_post("NotExistingPost");
        if (isError(res)) {
            expect(res.http_status_code).toBe(404);
        } else {
            throw new Error("Failed");
        }
    });
    test("Get Single HTML Post", async () => {
        const res = await pbe.get_post(post_slug, false);
        if (!isError(res)) {
            expect(res.metadata.slug).toBe("test");
        } else {
            throw new Error("Failed");
        }
    });
    test("Get Public Posts", async () => {
        const res = await pbe.get_posts(0);
        if (!isError(res)) {
            expect(res[0].slug).toBe("test");
        } else {
            throw new Error("Failed");
        }
    });
    test("Get Posts with Tags", async () => {
        const res = await pbe.get_posts_with_tag("test");
        if (!isError(res)) {
            expect(res[0].slug).toBe("test");
        } else {
            throw new Error("Failed");
        }
    });
    test("Search", async () => {
        const res = await pbe.search("*");
        if (!isError(res)) {
            expect(res[0].slug).toBe("test");
        } else {
            throw new Error("Failed");
        }
    });
    test("Submit Feedback", async () => {
        const res = await pbe.submit_feedback(true, post_slug, "lol")
        console.log(res)
        if (res !== null) {
            throw new Error("Failed");
        }
    })
    test("Submit Feedback Not Found", async () => {
        const res = await pbe.submit_feedback(true, "post_slug")
        if (isError(res)) {
            expect(res.http_status_code).toBe(404)
        } else {
            throw new Error("Failed");
        }
    })
    test("Submit Feedback Not Found", async () => {
        const res = await pbe.submit_feedback(true, post_slug)
        if (isError(res)) {
            expect(res.http_status_code).toBe(409)
        } else {
            throw new Error("Failed");
        }
    })
});
