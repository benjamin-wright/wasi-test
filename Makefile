CLUSTER_NAME ?= wasm-test
KUBECONFIG ?= .scratch/kubeconfig.yaml
REGISTRY_NAME ?= wasm-registry.localhost
REGISTRY_PATH ?= $(REGISTRY_NAME):5000/$(USER)
IMAGE ?= test-image
ARCH ?= $(shell uname -m)
ifeq ($(ARCH),arm64)
	ARCH=aarch64
endif

.PHONY: k3s-wasm
k3s-wasm:
	docker buildx build \
		-t k3s-wasm \
		--platform=linux/$(ARCH) \
		--build-arg ARCH=$(ARCH) \
		--output=type=docker \
		- < docker/k3s-wasm.Dockerfile

.PHONY: init
init:
	brew upgrade rustup
	brew upgrade kind
	brew upgrade k6

	rustup target add wasm32-wasi
	rustup update

	helm repo add cnpg https://cloudnative-pg.github.io/charts

.PHONY: start
start: k3s-wasm cluster infra

# See here: https://www.cncf.io/blog/2024/03/28/webassembly-on-kubernetes-the-practice-guide-part-02/
.PHONY: cluster
cluster:
	docker pull registry:2

	k3d cluster create $(CLUSTER_NAME) \
		--registry-create $(REGISTRY_NAME) \
		--image k3s-wasm \
		--kubeconfig-update-default=false

	mkdir -p .scratch
	k3d kubeconfig get $(CLUSTER_NAME) > $(KUBECONFIG)

.PHONY: infra
infra:
	helm upgrade --install cnpg cnpg/cloudnative-pg \
		--kubeconfig $(KUBECONFIG) \
		--namespace cnpg-system \
		--create-namespace

.PHONY: stop
stop:
	k3d cluster delete $(CLUSTER_NAME)
	rm -rf .scratch

.PHONY: build
build:
	cargo build --release
	mkdir -p bin
	mv target/wasm32-wasi/release/wasm.wasm bin/wasm.wasm

.PHONY: build-native
build-native:
	docker buildx build \
		-t $(IMAGE) \
		--output=type=docker \
		-f docker/rust.Dockerfile \
		--progress plain \
		.

.PHONY: image
image:
	docker buildx build \
		-t $(IMAGE) \
		-f docker/wasm.Dockerfile \
		--output=type=docker \
		bin

.PHONY: docker
docker:
	docker run --rm -p 8080:8080 -p 8081:8081 --runtime=io.containerd.wasmedge.v1 $(IMAGE)
