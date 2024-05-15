custom_build(
    'wasm',
    'IMAGE=$EXPECTED_REF make build image',
    ['.'],
)

k8s_yaml(helm('./chart', set=['name=wasm', 'image=wasm']))

k8s_resource(
    'wasm',
    trigger_mode = TRIGGER_MODE_MANUAL,
    port_forwards = [8080],
    labels = ['app']
)

local_resource(
    'test',
    'curl http://localhost:8080/health',
    auto_init = False,
    trigger_mode = TRIGGER_MODE_MANUAL,
    labels = ['scripts']
)

local_resource(
    'load - 50 for 1m',
    'k6 run -q load/50-users-1m.js',
    auto_init = False,
    trigger_mode = TRIGGER_MODE_MANUAL,
    labels = ['scripts']
)