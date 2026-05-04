$ErrorActionPreference = "Stop"

$ScriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Resolve-Path (Join-Path $ScriptRoot "..\..")
Set-Location $ProjectRoot

cargo test --manifest-path ".\Lessons\Lesson_05_UnitTests\Cargo.toml"
exit $LASTEXITCODE
