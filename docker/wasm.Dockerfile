FROM scratch

COPY wasm_aot.wasm /wasm

ENTRYPOINT [ "/wasm" ]
