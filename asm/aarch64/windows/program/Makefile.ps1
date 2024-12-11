Param(
    [Parameter()]
    [string]$Name = "program",
    [Parameter()]
    [switch]$IsDebug = $PSBoundParameters.ContainsKey("Debug"),
    [Parameter()]
    [switch]$Clean
)

$CFLAGS = ""
$LFLAGS = ""

function Find-Vsdevcmd {
    $ProgramFilesDir = Split-Path -Parent ${Env:CommonProgramFiles(x86)}
    $VsWhere = Join-Path -Path $ProgramFilesDir -ChildPath "Microsoft Visual Studio\Installer\vswhere.exe"
    $InstallDir = $(& "$VsWhere" "-latest", "-products", "*", "-requires", "Microsoft.VisualStudio.Component.VC.Tools.x86.x64", "-property", "installationPath")
    
    if(-Not $InstallDir) {
        Write-Host "Install directory not found" -ForegroundColor Red
        exit -1
    }

    $Vsdevcmd = Join-Path $InstallDir -ChildPath "Common7\Tools\VsDevCmd.bat"

    if(-Not (Test-Path $Vsdevcmd)) {
        Write-Host "VsDevCmd not found" -ForegroundColor Red
        exit -1
    }

    $Vsdevcmd
}

function Set-Environment {
    Param(
        [Parameter()]
        [string]$Vsdevcmd
    )

    Write-Host "Initializing environment..." -ForegroundColor Green

    & "${Env:COMSPEC}" /s /c "`"$Vsdevcmd`" -arch=arm64 -host_arch=amd64 -no_logo && set" | ForEach-Object {
        $lhs, $rhs = $_ -Split "=", 2
        Set-Content Env:\"$lhs" $rhs
    }
}

if($Clean) {
    Remove-Item -LiteralPath (Join-Path $PSScriptRoot -ChildPath "bin") -Force -Recurse
    Remove-Item -LiteralPath (Join-Path $PSScriptRoot -ChildPath "obj") -Force -Recurse
    Write-Host "Build directories removed" -ForegroundColor Yellow
    exit 0
}

Set-Environment -Vsdevcmd (Find-Vsdevcmd)

New-Item -Path $PSScriptRoot -Name "bin" -ItemType Directory -Force
New-Item -Path $PSScriptRoot -Name "obj" -ItemType Directory -Force

if($IsDebug) {
    armasm64 -machine ARM64 -g "${Name}.asm" -o obj/${Name}.obj $CFLAGS.Split(' ')
    link /DEBUG obj/${Name}.obj /OUT:bin/${Name}.exe $LFLAGS.Split(' ') /SUBSYSTEM:WINDOWS
} else {
    armasm64 -machine ARM64 "${Name}.asm" -o obj/${Name}.obj $CFLAGS.Split(' ')
    link obj/${Name}.obj /OUT:bin/${Name}.exe $LFLAGS.Split(' ') /SUBSYSTEM:WINDOWS
}
