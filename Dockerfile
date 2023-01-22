FROM rust:1.66

WORKDIR /app
COPY . .

RUN cargo build --release

EXPOSE 4002

CMD ["cargo", "run", "--release"]