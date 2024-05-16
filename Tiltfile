custom_build(
    'wasm-aot',
    'IMAGE=$EXPECTED_REF make image-aot',
    ['.'],
    image_deps = ['wasm']
)

k8s_yaml(helm('./chart', set=['name=wasm-aot', 'image=wasm-aot', 'runtimeClassName=wasmedge']))

k8s_resource(
    'wasm-aot',
    trigger_mode = TRIGGER_MODE_MANUAL,
    port_forwards = ["8084:8080"],
    labels = ['wasm-aot']
)

custom_build(
    'wasm',
    'IMAGE=$EXPECTED_REF make build image',
    ['.'],
)

k8s_yaml(helm('./chart', set=['name=wasm', 'image=wasm', 'runtimeClassName=wasmedge']))

k8s_resource(
    'wasm',
    trigger_mode = TRIGGER_MODE_MANUAL,
    port_forwards = [8080],
    labels = ['wasm']
)

custom_build(
    'rust',
    'IMAGE=$EXPECTED_REF make build-native',
    ['.'],
)

k8s_yaml(helm('./chart', set=['name=rust', 'image=rust']))

k8s_resource(
    'rust',
    trigger_mode = TRIGGER_MODE_MANUAL,
    port_forwards = ["8081:8080"],
    labels = ['rust']
)


custom_build(
    'golang',
    'IMAGE=$EXPECTED_REF make build-go image-go',
    ['.'],
)

k8s_yaml(helm('./chart', set=['name=golang', 'image=golang']))

k8s_resource(
    'golang',
    trigger_mode = TRIGGER_MODE_MANUAL,
    port_forwards = ["8082:8080"],
    labels = ['golang']
)

custom_build(
    'node',
    'IMAGE=$EXPECTED_REF make build-node',
    ['.'],
)

k8s_yaml(helm('./chart', set=['name=node', 'image=node']))

k8s_resource(
    'node',
    trigger_mode = TRIGGER_MODE_MANUAL,
    port_forwards = ["8083:8080"],
    labels = ['node']
)

local_resource(
    'test',
    'curl http://localhost:8080/health',
    auto_init = False,
    trigger_mode = TRIGGER_MODE_MANUAL,
    labels = ['scripts']
)

local_resource(
    'wasm load - 50 for 30s',
    'k6 run -q load/wasm-50-users-30s.js',
    auto_init = False,
    trigger_mode = TRIGGER_MODE_MANUAL,
    labels = ['wasm']
)

local_resource(
    'rust load - 50 for 30s',
    'k6 run -q load/rust-50-users-30s.js',
    auto_init = False,
    trigger_mode = TRIGGER_MODE_MANUAL,
    labels = ['rust']
)

local_resource(
    'go load - 50 for 30s',
    'k6 run -q load/go-50-users-30s.js',
    auto_init = False,
    trigger_mode = TRIGGER_MODE_MANUAL,
    labels = ['golang']
)

local_resource(
    'node load - 50 for 30s',
    'k6 run -q load/node-50-users-30s.js',
    auto_init = False,
    trigger_mode = TRIGGER_MODE_MANUAL,
    labels = ['node']
)