FROM node:17.8-bullseye as frontend

WORKDIR /usr/src/app
COPY frontend/package*.json ./
COPY frontend/pnpm-lock.yaml ./

RUN corepack enable && corepack prepare pnpm@7.1.0 --activate && pnpm i
COPY frontend/ .
RUN pnpm run build


FROM rust:1.67 AS builder

RUN update-ca-certificates

# Create appuser
ENV USER=pbe
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    #--home "/" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}" && \
    apt update &&  \
    apt install -y libpq-dev


WORKDIR /PersonalBlogEngine
RUN mkdir -p frontend/dist
COPY ./ .
COPY --from=frontend /usr/src/app/dist frontend/dist


# We no longer need to use the x86_64-unknown-linux-musl target
RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################
FROM debian:bullseye-slim

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /PersonalBlogEngine

# Copy our build
COPY --from=builder /PersonalBlogEngine/target/release/PersonalBlogEngine ./
RUN apt update && \
    apt install --no-install-recommends -y libpq-dev && \
    rm -rf /var/lib/apt/lists/*
# Use an unprivileged user.
USER pbe:pbe

CMD ["/PersonalBlogEngine/PersonalBlogEngine"]
