<#
.SYNOPSIS
Builds aic-sim and starts one run the safe way on this box.

.DESCRIPTION
Builds release, stops if the build failed (a failed build leaves the old exe in place),
copies the exe to out/<Name>.exe so later builds can't touch it or be blocked by it, and
starts the run at below-normal priority, detached, with stdout and stderr in
out/<Name>.out and out/<Name>.err. Prints the pid and returns. out/<Name>.exit appears
when the run ends, holding its exit code: wait on that.

.EXAMPLE
pwsh scripts/run_sim.ps1 s1 experiment experiments/gate.json --seed 3000 --threads 8 --out out/s1

.EXAMPLE
pwsh scripts/run_sim.ps1 -NoBuild s2 run configs/baseline.json --agent engine --games 64
#>
# No [Parameter] attributes: they would make `--out` an ambiguous common parameter. What
# isn't -NoBuild lands in $args for the sim.
param([string]$Name, [switch]$NoBuild)
$SimArgs = $args
if (-not $Name) { throw 'usage: run_sim.ps1 NAME [-NoBuild] ARGS...' }
$ErrorActionPreference = 'Stop'
Set-Location (Split-Path $PSScriptRoot -Parent)
New-Item -ItemType Directory -Force out | Out-Null

$exe = "out/$Name.exe"
if (-not $NoBuild) {
    cargo build --release -p aic-sim
    if ($LASTEXITCODE -ne 0) {
        throw "build failed (exit $LASTEXITCODE): nothing started, out/$Name.exe left as it was"
    }
}
Copy-Item target/release/aic-sim.exe $exe -Force
Remove-Item "out/$Name.exit" -ErrorAction SilentlyContinue

# The wrapper is what stays below normal and writes the exit marker: the sim inherits its
# priority, and the marker can't be missed by whoever waits, whatever way the sim ends.
$quoted = ($SimArgs | ForEach-Object { "'" + ($_ -replace "'", "''") + "'" }) -join ' '
$script = "& './$exe' $quoted > 'out/$Name.out' 2> 'out/$Name.err'; " +
          "Set-Content 'out/$Name.exit' `$LASTEXITCODE"
$encoded = [Convert]::ToBase64String([Text.Encoding]::Unicode.GetBytes($script))
$p = Start-Process pwsh -ArgumentList '-NoProfile', '-NonInteractive', '-EncodedCommand', $encoded -WindowStyle Hidden -PassThru
$p.PriorityClass = 'BelowNormal'
"started $Name, wrapper pid $($p.Id): $exe $($SimArgs -join ' ')"
