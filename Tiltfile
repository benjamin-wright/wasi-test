custom_build(
    'wasm',
    'IMAGE=$EXPECTED_REF make build image',
    ['.'],
)

k8s_yaml(helm('./chart', set=['name=wasm', 'image=wasm']))

k8s_resource(
    'wasm',
    trigger_mode = TRIGGER_MODE_MANUAL
)