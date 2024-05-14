CLUSTER_NAME ?= wasm-test
KUBECONFIG ?= .scratch/kubeconfig.yaml
REGISTRY_NAME ?= wasm-registry.localhost
REGISTRY_PATH ?= $(REGISTRY_NAME):5000/$(USER)

.PHONY: k3s-wasm
k3s-wasm:
	docker buildx build \
		-t k3s-wasm \
		--platform=linux/amd64 \
		--output=type=docker \
		- < docker/k3s-wasm.Dockerfile

.PHONY: init
init:
	brew upgrade rustup
	brew upgrade kind

	rustup target add wasm32-wasi
	rustup update

	helm repo add cnpg https://cloudnative-pg.github.io/charts

.PHONY: start
start: k3s-wasm cluster infra

# See here: https://www.cncf.io/blog/2024/03/28/webassembly-on-kubernetes-the-practice-guide-part-02/
.PHONY: cluster
cluster:
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

IMAGE ?= test-image
.PHONY: image
image:
	docker buildx build \
		-t $(IMAGE) \
		-f docker/wasm.Dockerfile \
		--output=type=docker \
		target/wasm32-wasi/release

.PHONY: docker
docker:
	docker run --rm -p 8080:8080 -p 8081:8081 --runtime=io.containerd.wasmedge.v1 $(IMAGE)
