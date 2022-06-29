# PersonalBlogEngine

A headless CMS for simple 1-person blogs.

![](https://img.shields.io/endpoint?url=https://time.mawoka.eu.org/api/compat/shields/v1/Mawoka/interval:any/project:PersonalBlogEngine&label=Time%20spent%20on%20PBE&style=for-the-badge)

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
- Docker (-compose)

## How to use it

1. Install it and set it up:
    1. Get
       the [`docker-compose.yml`](https://github.com/mawoka-myblock/PersonalBlogEngine/blob/master/docker-compose.yml): `wget -O docker-compose.yml https://raw.githubusercontent.com/mawoka-myblock/PersonalBlogEngine/master/docker-compose.yml`
    2. Add a reverse-proxy
    3. Open your browser at the `/admin/`-page.
       Note: The last `/` is required. Visiting `https://test.com/admin` will **NOT** work.
    4. Enter your email-address (doesn't have to be valid) and your password and press `SETUP`.
    5. Log in with your email and password
2. Integrate it into the frontend
    Use the TypeScipt library [@mawoka/pbe](https://www.npmjs.com/package/@mawoka/pbe).
3. Create your first post
    1. Go to your admin-page `https://test.com/admin/` and log in if you aren't already
    2. Click on `Create new post` and enter the slug of your new post. Note: **You won't be able to change the slug
       afterwards!**
    3. Enter a title, an intro and your markdown. After that, click `save`. Note: For now, you won't get any feedback
       from the save-button, but it works.
4. Done!
    
