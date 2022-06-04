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

export const isError = (input: any | Error): input is Error => {
    return (<Error>input).http_status_code !== undefined;
};

export class PBE {
    api_url: string;

    constructor(api_url: string) {
        this.api_url = api_url;
    }

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

    async search(
        query: string,
    ): Promise<Array<Metadata | null> | Error> {
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

    async submit_feedback(
        thumbs_up: boolean,
        post_slug: string,
        feedback?: string,
    ): Promise<null | Error> {
        try {
            await axios.post(`${this.api_url}/api/v1/feedback/`, {
                thumbs_up: thumbs_up,
                post_slug: post_slug,
                feedback: feedback
            })
            return null
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
