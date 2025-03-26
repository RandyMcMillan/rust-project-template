FROM rust:slim as ping
COPY . /usr/src/app
WORKDIR /usr/src/app
RUN cargo build --release
EXPOSE 3000
#VOLUME ["/usr/local/cargo"]
VOLUME ["/tmp"]
ENTRYPOINT ["target/release/ping-example"]
from ping as chat
ENTRYPOINT ["target/release/chat-project-example"]
