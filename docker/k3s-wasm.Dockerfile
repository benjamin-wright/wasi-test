FROM busybox as get

RUN wget https://github.com/containerd/runwasi/releases/download/containerd-shim-wasmedge%2Fv0.4.0/containerd-shim-wasmedge-x86_64.tar.gz -O /tmp/containerd-shim-wasmedge-x86_64.tar.gz && \
    tar -xvf /tmp/containerd-shim-wasmedge-x86_64.tar.gz -C /tmp

FROM rancher/k3s:v1.28.8-k3s1

COPY --from=get /tmp/containerd-shim-wasmedge-v1 /usr/local/bin/containerd-shim-wasmedge-v1
