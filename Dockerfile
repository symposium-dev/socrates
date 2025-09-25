# Build stage
FROM rust:1.89 as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

# Runtime stage
FROM public.ecr.aws/lambda/provided:al2

COPY --from=builder /app/target/release/socrates /var/runtime/bootstrap

CMD ["bootstrap"]