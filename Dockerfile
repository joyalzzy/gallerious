FROM lukemathwalker/cargo-chef:latest-rust-slim-bullseye AS chef
WORKDIR /app

FROM chef AS planner
COPY ./gal /app
RUN echo $(ls -a /app)
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS backend-build
ARG VITE_API_URL

COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY ./gal /app
RUN cargo build --release --bin gallerious

FROM node:current-alpine AS frontend-build
ARG VITE_API_URL
RUN echo $API_URL
WORKDIR /app

COPY galleri /app/galleri
RUN cd galleri \
  && npm i npm-run-all \
  && npm run build

FROM nginx:stable
ARG VITE_API_URL
RUN apt update \
   && apt-get install -y libc6

WORKDIR /app

# Copy from backend stage
COPY --from=backend-build /app/target/release/gallerious /usr/bin


# Copy from frontend stage
COPY --from=frontend-build /app/galleri/dist /app/dist

# Copy entrypoint, nginx config, and DB
COPY entrypoint.sh /usr/bin/entrypoint.sh
COPY nginx.conf /etc/nginx/nginx.conf

EXPOSE 80

CMD ["/usr/bin/entrypoint.sh"]