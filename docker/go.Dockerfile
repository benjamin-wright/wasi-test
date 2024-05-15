FROM scratch

COPY go /app

ENTRYPOINT [ "/app" ]
