FROM debian:latest

RUN apt-get update && apt-get install -y \
    g++ \
    libcrypto++-dev

WORKDIR /app
COPY chain.cpp main.cpp ./

RUN g++ chain.cpp main.cpp -o app -lcryptopp

CMD ["./app"]
