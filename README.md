# PersonalBlogEngine

A headless CMS for simple 1-person blogs.

## Idea

I want a headless CMS for my blog, so I've used Strapi.
I think it's not the right tool for the job.
I think that it is
way too complex and the setup isn't easy.
The deployment is also annoying, because you have to use yarn extern, etc.
It's just not fun.
The updates: They are also way too complex. Not like `docker pull strapi/strapi:latest`. No, build your own
docker-image. Then I thought:
> Why not create your own headless blog-cms, with all the features you need in Rust, so I can even learn more rust?

That's it.

## Features
- A simple web-ui
- An easy to use REST-API
- Included full-text search
- A tag-system
- One binary

## Requirements
- Your own frontend
- A PostgreSQL database
- A webserver