FROM rust:latest AS build
WORKDIR /home/Work/actix_web-diesel-swagger-boilerplate
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --package actix_web-diesel-swagger-boilerplate --release
EXPOSE 5002
FROM ubuntu:22.04
RUN apt update && apt install -y libssl-dev libpq-dev ca-certificates
COPY --from=build /home/Work/actix_web-diesel-swagger-boilerplate/target/release/actix_web-diesel-swagger-boilerplate /opt/actix_web-diesel-swagger-boilerplate/
WORKDIR /opt/actix_web-diesel-swagger-boilerplate/
ENTRYPOINT ["/opt/actix_web-diesel-swagger-boilerplate/actix_web-diesel-swagger-boilerplate"]
CMD ["actix_web-diesel-swagger-boilerplate"]