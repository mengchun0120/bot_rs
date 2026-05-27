param (
    [string]$map
)

$env:RUST_LOG="bot_rs=info";  $env:WGPU_BACKEND="D3D12"; cargo run -- --mode run-game --log game.log --game-config assets\config\game_config.json --map "$map"
