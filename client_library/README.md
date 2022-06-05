# PersonalBlogEngine

This is the client-library for [PBE](https://github.com/mawoka-myblock/personalblogengine/), a free and easy-to-use
headless CMS!

## Getting Started

### Requirements

- A running instance of PBE (See [here](https://github.com/mawoka-myblock/personalblogengine/#how-to-use-it))
- Your own frontend for your blog

### Setup

1. Install the library: `npm i @mawoka/pbe`
2. Use it
```typescript
import {PBE, isError} from "@mawoka/pbe"

const pbe = new PBE("https://your_pbe_install.test") // Note: There isn't a "/" at the end of the url
// Get a single post via a slug
const post = pbe.get_post("your_slug", false) // You can either get the HTML (false) or the markdown (true)
// Now, check if the post isn't an error:
if (isError(post)) {
    // Get the status_code the api returned:
    const code = post.http_status_code
    if (code === 404) {
        alert("Post not found")
    } else {
        alert("Unknown error!")
    }
}
```
3. Look at the [API docs](https://pbe-docs.netlify.app/classes/pbe)