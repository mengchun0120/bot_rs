param (
    $map
)

$env:RUST_LOG="bot_rs=info";  $env:WGPU_BACKEND="D3D12"; cargo run -- --app-mode gen-map -l gen_map.log -c assets\config\gen_map_config.json -m "$map"
