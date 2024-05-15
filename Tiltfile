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
    port_forwards = [8081],
    labels = ['rust']
)

local_resource(
    'test',
    'curl http://localhost:8080/health',
    auto_init = False,
    trigger_mode = TRIGGER_MODE_MANUAL,
    labels = ['scripts']
)

local_resource(
    'wasm load - 50 for 1m',
    'k6 run -q load/wasm-50-users-1m.js',
    auto_init = False,
    trigger_mode = TRIGGER_MODE_MANUAL,
    labels = ['wasm']
)

local_resource(
    'rust load - 50 for 1m',
    'k6 run -q load/rust-50-users-1m.js',
    auto_init = False,
    trigger_mode = TRIGGER_MODE_MANUAL,
    labels = ['rust']
)