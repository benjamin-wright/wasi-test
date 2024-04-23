.PHONY: init
init:
	brew upgrade rustup
	brew upgrade kind
	brew upgrade knative/client/kn

	rustup target add wasm32-wasi
	rustup update

	helm repo add kwasm http://kwasm.sh/kwasm-operator/

.PHONY: cluster
cluster:
	kn quickstart kind --install-serving
	kubectl --context kind-knative patch  configmap config-deployment -n knative-serving -p '{"data": {"registries-skipping-tag-resolving": "localhost:5001"} }'
	helm upgrade --kube-context kind-knative --install -n kwasm --create-namespace kwasm-operator kwasm/kwasm-operator
	kubectl --context kind-knative annotate node --all --overwrite kwasm.sh/kwasm-node=true
	kubectl --context kind-knative patch configmap config-features -n knative-serving -p '{"data": {"kubernetes.podspec-runtimeclassname": "enabled"} }'

.PHONY: teardown
teardown:
	-kind delete cluster --name knative
	-docker stop kind-registry
	-docker rm kind-registry

.PHONY: build
build:
	cargo build
