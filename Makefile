CLUSTER_NAME ?= wasm-test
KUBECONFIG ?= .scratch/kubeconfig.yaml
REGISTRY_NAME ?= wasm-registry.localhost
REGISTRY_PATH ?= $(REGISTRY_NAME):5000/$(USER)

.PHONY: init
init:
	brew upgrade rustup
	brew upgrade k3d

	rustup target add wasm32-wasi
	rustup update

	helm repo add kwasm http://kwasm.sh/kwasm-operator/
	helm repo add cnpg https://cloudnative-pg.github.io/charts

.PHONY: start
start: registry cluster infra

.PHONY: registry
registry:
	if ! k3d registry list | grep $(REGISTRY_NAME); then \
		k3d registry create $(REGISTRY_NAME) --port 5000; \
	fi

.PHONY: cluster
cluster:
	K3D_FIX_DNS=1 k3d cluster create $(CLUSTER_NAME) \
		--servers 1 \
		--registry-use $(REGISTRY_NAME):5000 \
		--port 80:80@loadbalancer \
		--wait

	mkdir -p .scratch
	k3d kubeconfig get $(CLUSTER_NAME) > $(KUBECONFIG)
	chmod 600 $(KUBECONFIG)

.PHONY: infra
infra:
	helm upgrade --install kwasm-operator kwasm/kwasm-operator \
		--kubeconfig $(KUBECONFIG) \
		--namespace kwasm \
		--create-namespace

	helm upgrade --install cnpg cnpg/cloudnative-pg \
		--kubeconfig $(KUBECONFIG) \
		--namespace cnpg-system \
		--create-namespace

.PHONY: stop
stop:
	k3d cluster delete $(CLUSTER_NAME)
	k3d registry delete $(REGISTRY_NAME)
	rm -rf .scratch

ifdef CI
BUILD_ARGS := --release
endif

.PHONY: build
build:
	cargo build $(BUILD_ARGS)

IMAGE ?= test-image
.PHONY: image
image:
	docker buildx build \
		-t $(IMAGE) \
		--platform=wasi/wasm32 \
		-f docker/wasm.Dockerfile \
		--output=type=docker \
		target/wasm32-wasi/release

.PHONY: docker
docker:
	docker run --rm -p 8081:8080 --runtime=io.containerd.wasmedge.v1 --platform=wasi/wasm32 $(IMAGE)
