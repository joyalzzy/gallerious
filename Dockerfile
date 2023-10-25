FROM rustlang/rust:nightly-slim AS backend-build
ARG VITE_API_URL
RUN apt-get update 

WORKDIR /app

COPY gal /app/gal
RUN cd gal \
  && cargo build --release 

FROM node:current-alpine AS frontend-build
ARG VITE_API_URL
ENV VITE_API_URL "localhost:8080/v1"
RUN echo $API_URL
WORKDIR /app

COPY galleri /app/galleri
RUN cd galleri \
  && npm run build

FROM nginx:stable
ARG VITE_API_URL
RUN apt update

WORKDIR /app

# Copy from backend stage
COPY --from=backend-build /app/gal/target/release/gallerious /usr/bin


# Copy from frontend stage
COPY --from=frontend-build /app/galleri/dist /app/dist

# Copy entrypoint, nginx config, and DB
COPY entrypoint.sh /usr/bin/entrypoint.sh
COPY nginx.conf /etc/nginx/nginx.conf

EXPOSE 80

CMD ["/usr/bin/entrypoint.sh"]