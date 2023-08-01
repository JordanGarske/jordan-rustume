FROM rust:1.71 as build
# Install the nightly toolchain
RUN rustup default nightly-2023-04-01
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=6666
# Set the working directory for the build stage
WORKDIR /app

# Copy the entire content of your Rust application into the container's /app directory
COPY . .
RUN  apt-get install -y openssl
# Change the working directory to /app/rustume/src and build the Rust application using Cargo
WORKDIR /app/rustume/src
RUN cargo build --release
RUN apt-get update && apt-get install -y libpq-dev
# Stage 2: Create the final minimal image
 FROM debian:buster

# Copy the binary from the build stage to the final image
COPY --from=build /app/rustume/target/release/rustume /usr/local/bin/rustume

# Set the working directory in the final image (optional, not necessary for binary execution)
WORKDIR /usr/local/bin
RUN apt-get update && apt-get install -y openssl

# libpq related (required by diesel)
COPY --from=build /usr/lib/x86_64-linux-gnu/libpq.so* /usr/lib/x86_64-linux-gnu/
COPY --from=build /usr/lib/x86_64-linux-gnu/libgssapi_krb5.so* /usr/lib/x86_64-linux-gnu/
COPY --from=build /usr/lib/x86_64-linux-gnu/libldap_r-2.4.so* /usr/lib/x86_64-linux-gnu/
COPY --from=build /usr/lib/x86_64-linux-gnu/libkrb5.so* /usr/lib/x86_64-linux-gnu/
COPY --from=build /usr/lib/x86_64-linux-gnu/libk5crypto.so* /usr/lib/x86_64-linux-gnu/
COPY --from=build /usr/lib/x86_64-linux-gnu/libkrb5support.so* /usr/lib/x86_64-linux-gnu/
COPY --from=build /usr/lib/x86_64-linux-gnu/liblber-2.4.so* /usr/lib/x86_64-linux-gnu/
COPY --from=build /usr/lib/x86_64-linux-gnu/libsasl2.so* /usr/lib/x86_64-linux-gnu/
COPY --from=build /usr/lib/x86_64-linux-gnu/libgnutls.so* /usr/lib/x86_64-linux-gnu/
COPY --from=build /usr/lib/x86_64-linux-gnu/libp11-kit.so* /usr/lib/x86_64-linux-gnu/
COPY --from=build /usr/lib/x86_64-linux-gnu/libidn2.so* /usr/lib/x86_64-linux-gnu/
COPY --from=build /usr/lib/x86_64-linux-gnu/libunistring.so* /usr/lib/x86_64-linux-gnu/
COPY --from=build /usr/lib/x86_64-linux-gnu/libtasn1.so* /usr/lib/x86_64-linux-gnu/
COPY --from=build /usr/lib/x86_64-linux-gnu/libnettle.so* /usr/lib/x86_64-linux-gnu/
COPY --from=build /usr/lib/x86_64-linux-gnu/libhogweed.so* /usr/lib/x86_64-linux-gnu/
COPY --from=build /usr/lib/x86_64-linux-gnu/libgmp.so* /usr/lib/x86_64-linux-gnu/
COPY --from=build /usr/lib/x86_64-linux-gnu/libffi.so* /usr/lib/x86_64-linux-gnu/
COPY --from=build /lib/x86_64-linux-gnu/libcom_err.so* /lib/x86_64-linux-gnu/
COPY --from=build /lib/x86_64-linux-gnu/libkeyutils.so* /lib/x86_64-linux-gnu/

# Specify the command to run your application (replace "rustume" with the actual binary name)
CMD ["rustume"]