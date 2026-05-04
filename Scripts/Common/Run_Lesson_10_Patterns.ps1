$ErrorActionPreference = "Stop"

$ScriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Resolve-Path (Join-Path $ScriptRoot "..\..")
Set-Location $ProjectRoot

cargo run --manifest-path ".\Lessons\Lesson_10_Patterns\Cargo.toml"
exit $LASTEXITCODE
