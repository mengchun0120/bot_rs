param (
    $map
)

$env:RUST_LOG="bot_rs=info";  $env:WGPU_BACKEND="D3D12"; cargo run -- --mode gen-map --log gen_map.log --game-config assets\config\game_config.json --gen-map-config assets\config\gen_map_config.json --map "$map"
