import axios from "axios";

export interface PostData {
    content: string;
    metadata: Metadata;
}

export interface Error {
    message: string;
    http_status_code: number;
}

export interface Metadata {
    slug: string;
    title: string;
    created_at: string;
    updated_at: string;
    tags: string[];
    intro: string;
    published: boolean;
    thumbs_up: number;
    thumbs_down: number;
}

export interface SearchResult {
    slug: string;
    title: string;
    intro: string;
    tags: string[];
}

export const isError = (input: any | Error): input is Error => {
    return (<Error>input).http_status_code !== undefined;
};

export class PBE {
    private readonly api_url: string;

    /**
     * The constructor function is a special function that is called when an object is created from a class
     * @param {string} api_url - The URL of the API you want to use.
     */
    constructor(api_url: string) {
        this.api_url = api_url;
    }

    /**
     * It takes a slug and a boolean, and returns a promise that resolves to a PostData object or an Error object
     * @param {string} slug - The slug of the post you want to get.
     * @param [raw=false] - boolean - If true, the raw markdown will be returned. If false, the rendered HTML will be
     * returned.
     * @returns An PostData object or an Error object.
     **/
    async get_post(slug: string, raw = false): Promise<PostData | Error> {
        try {
            const res = await axios.get(
                raw
                    ? `${this.api_url}/api/v1/public/raw`
                    : `${this.api_url}/api/v1/public/rendered`,
                {
                    params: {
                        slug: slug,
                    },
                }
            );
            const json_data = res.data;
            return {
                content: json_data.content,
                metadata: {
                    slug: json_data.metadata.slug,
                    created_at: json_data.metadata.created_at,
                    intro: json_data.metadata.intro,
                    tags: json_data.metadata.tags,
                    published: json_data.metadata.published,
                    thumbs_down: json_data.metadata.thumbs_down,
                    thumbs_up: json_data.metadata.thumbs_up,
                    title: json_data.metadata.title,
                    updated_at: json_data.metadata.updated_at,
                },
            };
        } catch (error) {
            if (error.response) {
                return {
                    message: "Unexpected Response",
                    http_status_code: error.response.status,
                };
            } else {
                throw error;
            }
        }
    }

    /**
     * It returns an array of post metadata, or an error
     * @param [offset=0] - The offset of the posts to get.
     * @returns An array of Metadata objects or an Error object.
     */
    async get_posts(offset = 0): Promise<Array<Metadata | null> | Error> {
        try {
            const res = await axios.get(`${this.api_url}/api/v1/public/posts`, {
                params: {
                    offset: offset,
                },
            });
            return res.data;
        } catch (error) {
            if (error.response) {
                return {
                    message: "Unexpected Response",
                    http_status_code: error.response.status,
                };
            } else {
                throw error;
            }
        }
    }

    /**
     * Get every post containing the given tag
     *
     * @param {string} tag - The tag you want to search for.
     * @param [offset=0] - The offset of the first post to return.
     * @returns An array of metadata objects or an error object.
     */
    async get_posts_with_tag(
        tag: string,
        offset = 0
    ): Promise<Array<Metadata | null> | Error> {
        try {
            const res = await axios.get(
                `${this.api_url}/api/v1/public/tags/${tag}`,
                {
                    params: {
                        offset: offset,
                    },
                }
            );
            return res.data;
        } catch (error) {
            if (error.response) {
                return {
                    message: "Unexpected Response",
                    http_status_code: error.response.status,
                };
            } else {
                throw error;
            }
        }
    }

    /**
     * It takes a query string and returns an array of metadata objects or an error
     * @param {string} query - The query string to search for.
     * @returns An array of Metadata objects or an Error object.
     */
    async search(query: string): Promise<Array<SearchResult | null> | Error> {
        try {
            const res = await axios.get(
                `${this.api_url}/api/v1/public/search`,
                {
                    params: {
                        query: query,
                    },
                }
            );
            return res.data;
        } catch (error) {
            if (error.response) {
                return {
                    message: "Unexpected Response",
                    http_status_code: error.response.status,
                };
            } else {
                throw error;
            }
        }
    }

    /**
     * It sends a POST request to the API endpoint `/api/v1/feedback/` with the parameters `thumbs_up`, `post_slug`, and
     * `feedback`
     * @param {boolean} thumbs_up - boolean - True if the user liked the post, false if they didn't.
     * @param {string} post_slug - The slug of the post you want to submit feedback for.
     * @param {string} [feedback] - string - The feedback text, which is optional
     * @returns An object with a message and http_status_code
     */
    async submit_feedback(
        thumbs_up: boolean,
        post_slug: string,
        feedback?: string
    ): Promise<null | Error> {
        try {
            await axios.post(`${this.api_url}/api/v1/feedback/`, {
                thumbs_up: thumbs_up,
                post_slug: post_slug,
                feedback: feedback,
            });
            return null;
        } catch (error) {
            if (error.response) {
                return {
                    message: "Unexpected Response",
                    http_status_code: error.response.status,
                };
            } else {
                throw error;
            }
        }
    }
}
