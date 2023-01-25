FROM --platform=$BUILDPLATFORM ghcr.io/arrow-air/tools/arrow-rust:latest AS build

ENV CARGO_INCREMENTAL=1
ENV RUSTC_BOOTSTRAP=0

COPY . /usr/src/app

# perl and build-base are needed to build openssl, see:
# https://github.com/openssl/openssl/blob/master/INSTALL.md#prerequisites
RUN apk -U add perl build-base ; cd /usr/src/app ; cargo build --release --features=vendored-openssl
FROM --platform=$TARGETPLATFORM alpine:latest
ARG PACKAGE_NAME=
COPY --from=build /usr/src/app/target/release/${PACKAGE_NAME} /usr/local/bin/${PACKAGE_NAME}
RUN ln -s /usr/local/bin/${PACKAGE_NAME} /usr/local/bin/server

ENTRYPOINT ["/usr/local/bin/server"]
