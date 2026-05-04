$ErrorActionPreference = "Stop"

$ScriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Resolve-Path (Join-Path $ScriptRoot "..\..")
Set-Location $ProjectRoot

cargo run --manifest-path ".\Lessons\Lesson_07_Generics\Cargo.toml"
exit $LASTEXITCODE
