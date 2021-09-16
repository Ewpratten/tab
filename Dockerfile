FROM debian:latest

WORKDIR /app

COPY ./target/release/tab /app/tab

RUN apt-get update -y
RUN apt-get install -y ffmpeg curl python
RUN curl -L https://yt-dl.org/downloads/latest/youtube-dl -o /usr/local/bin/youtube-dl
RUN chmod a+rx /usr/local/bin/youtube-dl

COPY ./docker-entrypoint.sh ./docker-entrypoint.sh

CMD ["/app/docker-entrypoint.sh"]