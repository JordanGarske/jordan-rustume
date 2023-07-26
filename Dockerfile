FROM rust:1.69
RUN rustup default nightly
WORKDIR /app

COPY . .
cd 
EXPOSE 3000
CMD ["myapp"]

from gcr.io/distroless/cc-debian10