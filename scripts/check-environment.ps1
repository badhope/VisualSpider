param(
    [switch]$Install,
    [switch]$CheckOnly,
    [switch]$Silent
)

$ErrorActionPreference = "Continue"

function Write-Status {
    param([string]$Message, [string]$Type = "Info")
    if (-not $Silent) {
        switch ($Type) {
            "Success" { Write-Host $Message -ForegroundColor Green }
            "Error" { Write-Host $Message -ForegroundColor Red }
            "Warning" { Write-Host $Message -ForegroundColor Yellow }
            "Info" { Write-Host $Message -ForegroundColor Cyan }
            default { Write-Host $Message }
        }
    }
}

function Test-Command {
    param([string]$Command)
    try {
        $null = Get-Command $Command -ErrorAction Stop
        return $true
    } catch {
        return $false
    }
}

function Test-Rust {
    Write-Status "检查 Rust 环境..." "Info"
    if (Test-Command "rustc") {
        $version = rustc --version 2>$null
        Write-Status "  ✓ Rust 已安装: $version" "Success"
        return $true
    } else {
        Write-Status "  ✗ Rust 未安装" "Error"
        return $false
    }
}

function Test-NodeJS {
    Write-Status "检查 Node.js 环境..." "Info"
    if (Test-Command "node") {
        $version = node --version 2>$null
        Write-Status "  ✓ Node.js 已安装: $version" "Success"
        return $true
    } else {
        Write-Status "  ✗ Node.js 未安装" "Error"
        return $false
    }
}

function Test-NPM {
    Write-Status "检查 NPM 环境..." "Info"
    if (Test-Command "npm") {
        $version = npm --version 2>$null
        Write-Status "  ✓ NPM 已安装: $version" "Success"
        return $true
    } else {
        Write-Status "  ✗ NPM 未安装" "Error"
        return $false
    }
}

function Test-VSBuildTools {
    Write-Status "检查 Visual Studio Build Tools..." "Info"
    $vsWhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
    if (Test-Path $vsWhere) {
        $installPath = & $vsWhere -latest -property installationPath 2>$null
        if ($installPath) {
            Write-Status "  ✓ Visual Studio Build Tools 已安装" "Success"
            return $true
        }
    }
    
    $msbuild = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\2022\BuildTools\MSBuild\Current\Bin\MSBuild.exe"
    if (Test-Path $msbuild) {
        Write-Status "  ✓ Visual Studio Build Tools 已安装" "Success"
        return $true
    }
    
    $msbuild2019 = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\2019\BuildTools\MSBuild\Current\Bin\MSBuild.exe"
    if (Test-Path $msbuild2019) {
        Write-Status "  ✓ Visual Studio Build Tools 已安装" "Success"
        return $true
    }
    
    Write-Status "  ✗ Visual Studio Build Tools 未安装" "Warning"
    return $false
}

function Test-WebView2 {
    Write-Status "检查 WebView2 运行时..." "Info"
    $regPath = "HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}"
    $regPathUser = "HKCU:\SOFTWARE\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}"
    
    if ((Test-Path $regPath) -or (Test-Path $regPathUser)) {
        Write-Status "  ✓ WebView2 运行时已安装" "Success"
        return $true
    } else {
        Write-Status "  ✗ WebView2 运行时未安装" "Warning"
        return $false
    }
}

function Install-Rust {
    Write-Status "正在安装 Rust..." "Info"
    try {
        $result = winget install Rustlang.Rustup --accept-source-agreements --accept-package-agreements
        if ($LASTEXITCODE -eq 0) {
            Write-Status "  ✓ Rust 安装成功" "Success"
            $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
            return $true
        }
    } catch {
        Write-Status "  ✗ Rust 安装失败: $_" "Error"
    }
    return $false
}

function Install-NodeJS {
    Write-Status "正在安装 Node.js..." "Info"
    try {
        $result = winget install OpenJS.NodeJS.LTS --accept-source-agreements --accept-package-agreements
        if ($LASTEXITCODE -eq 0) {
            Write-Status "  ✓ Node.js 安装成功" "Success"
            $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
            return $true
        }
    } catch {
        Write-Status "  ✗ Node.js 安装失败: $_" "Error"
    }
    return $false
}

function Install-VSBuildTools {
    Write-Status "正在安装 Visual Studio Build Tools..." "Info"
    Write-Status "  这可能需要几分钟时间，请耐心等待..." "Warning"
    try {
        $result = winget install Microsoft.VisualStudio.2022.BuildTools --override "--wait --passive --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended" --accept-source-agreements --accept-package-agreements
        if ($LASTEXITCODE -eq 0) {
            Write-Status "  ✓ Visual Studio Build Tools 安装成功" "Success"
            return $true
        }
    } catch {
        Write-Status "  ✗ Visual Studio Build Tools 安装失败: $_" "Error"
    }
    return $false
}

function Install-WebView2 {
    Write-Status "正在安装 WebView2 运行时..." "Info"
    try {
        $result = winget install Microsoft.EdgeWebView2Runtime --accept-source-agreements --accept-package-agreements
        if ($LASTEXITCODE -eq 0) {
            Write-Status "  ✓ WebView2 运行时安装成功" "Success"
            return $true
        }
    } catch {
        Write-Status "  ✗ WebView2 运行时安装失败: $_" "Error"
    }
    return $false
}

function Install-Dependencies {
    Write-Status "`n========================================" "Info"
    Write-Status "开始安装缺失的依赖..." "Info"
    Write-Status "========================================`n" "Info"
    
    $needsRestart = $false
    
    if (-not (Test-Rust)) {
        if (Install-Rust) {
            $needsRestart = $true
        }
    }
    
    if (-not (Test-NodeJS)) {
        if (Install-NodeJS) {
            $needsRestart = $true
        }
    }
    
    if (-not (Test-VSBuildTools)) {
        if (Install-VSBuildTools) {
            $needsRestart = $true
        }
    }
    
    if (-not (Test-WebView2)) {
        Install-WebView2 | Out-Null
    }
    
    Write-Status "`n========================================" "Info"
    Write-Status "依赖安装完成!" "Success"
    Write-Status "========================================`n" "Info"
    
    if ($needsRestart) {
        Write-Status "注意: 某些安装需要重启终端或重新登录才能生效" "Warning"
    }
    
    return $true
}

function Main {
    Write-Status "`n========================================" "Info"
    Write-Status "Windows 工具箱 - 环境检测" "Info"
    Write-Status "========================================`n" "Info"
    
    $allPassed = $true
    $results = @{}
    
    $results.Rust = Test-Rust
    $results.NodeJS = Test-NodeJS
    $results.NPM = Test-NPM
    $results.VSBuildTools = Test-VSBuildTools
    $results.WebView2 = Test-WebView2
    
    $allPassed = $results.Values -notcontains $false
    
    Write-Status "`n----------------------------------------" "Info"
    Write-Status "检测结果汇总:" "Info"
    Write-Status "----------------------------------------" "Info"
    
    foreach ($key in $results.Keys) {
        $status = if ($results[$key]) { "✓ 通过" } else { "✗ 缺失" }
        $color = if ($results[$key]) { "Success" } else { "Error" }
        Write-Status "  $key : $status" $color
    }
    
    if ($allPassed) {
        Write-Status "`n所有环境检查通过，可以正常运行应用!" "Success"
        return 0
    } else {
        Write-Status "`n检测到缺失的依赖项" "Warning"
        
        if ($Install -or (-not $CheckOnly)) {
            $response = if ($Silent) { "Y" } else { 
                Read-Host "是否自动安装缺失的依赖? (Y/N)"
            }
            if ($response -eq "Y" -or $response -eq "y") {
                Install-Dependencies
                return 0
            }
        }
        return 1
    }
}

exit Main
