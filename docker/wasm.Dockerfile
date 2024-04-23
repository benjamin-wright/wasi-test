FROM scratch

COPY . /wasm

ENTRYPOINT [ "/wasm" ]
