FROM --platform=$TARGETPLATFORM alpine:latest

WORKDIR /usr/src/app

RUN apk add --no-cache libc-dev git gcc librtlsdr-dev make
RUN git clone https://github.com/antirez/dump1090.git
RUN cd dump1090 && make

WORKDIR /usr/src/app/dump1090
