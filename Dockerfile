FROM rust:1-bullseye AS rust_builder
WORKDIR /src

# general rule: you should generally do things that don't change very often earlier in the Dockerfile than things that do change more often.
# we copy the source code later.
# because our dependency doesn't change often, but our code does.
# install our deps
COPY Cargo.toml Cargo.lock ./

# then copy our source code
COPY . ./

RUN cargo build --release

# build tailwind
FROM node:22-bullseye-slim AS node_builder
WORKDIR /src

COPY package.json package-lock.json ./
COPY ./tailwind.config.js ./

COPY ./styles/ ./styles/
COPY ./templates/ ./templates/

RUN npm ci
RUN npm run build

# Final stage run the app
FROM gcr.io/distroless/cc-debian12
WORKDIR /app

COPY --from=rust_builder ./src/target/release/canvas ./
COPY --from=node_builder ./src/public ./public/

CMD ["./canvas"]

