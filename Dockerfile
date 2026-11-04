# The docs site (site/) as a container: a native Topcoat server plus the asset
# bundle it loads from `assets/` beside the binary.
#
# Build from this directory, since the site depends on the registry crate one
# level up (`neobrutalism = { path = ".." }`) and on `themes/`:
#
#   docker build -t neobrutalism-site .
#   docker run --rm -p 3000:80 neobrutalism-site
#
# Vercel (Container runtime), Railway, Render, Fly.io and Cloud Run can all
# build this file from the repository root.

FROM rust:1.98-trixie AS build

# `topcoat asset bundle` writes the asset bundle the server loads at runtime.
RUN cargo install topcoat-cli --version 0.10.0 --locked

WORKDIR /app
COPY . .

# build.rs downloads Tailwind and the Lucide icon set, so this needs network.
WORKDIR /app/site
RUN cargo build --release --locked \
    && topcoat asset bundle --release

FROM gcr.io/distroless/cc-debian13:nonroot

WORKDIR /app
COPY --from=build /app/site/target/release/neobrutalism-site ./
COPY --from=build /app/site/target/release/assets ./assets

# `topcoat::start` binds HOST:PORT (default 127.0.0.1:3000). Vercel routes to
# port 80 unless PORT is set in the project settings; Railway, Render and
# Cloud Run inject their own PORT.
ENV HOST=0.0.0.0 \
    PORT=80
EXPOSE 80

CMD ["/app/neobrutalism-site"]
