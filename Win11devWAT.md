# Local Windows 11 Developer Environment Setup Guide

This guide establishes the local, zero-dependency, and non-bloated developer environment on Windows 11 to support the **Sovereign Interleaved Compilation Path** of the cdqn ecosystem. In accordance with the principles of **Origin Iteration**, we configure a lightweight, unprivileged command-line environment to compile, execute, and profile the strictly typed, memory-free **$Wat_{\text{safe}}$** Stage 0 seed compiler ($C_0$) and the **$QnIR$** targets. 

By executing everything locally on standard silicon (Option C), we bypass the performance-counter virtualization limits and build-handshake latencies of remote cloud CI/CD pipelines (such as GitHub Actions), establishing the raw, physical telemetry required for our **Variation-Evaluation-Selection (VES)** closed-loop.

---

## I. Setting Up the Windows Package Manager

To prevent the administrative popups, graphical wizard installers, and system registry pollution associated with legacy Windows installation methods, we use **Scoop** as our command-line package manager.

### A. Why Scoop?
Unlike Chocolatey or Winget, Scoop installs programs as self-contained, portable binaries directly inside the user's home directory (`~\scoop`), requiring zero administrator privileges. This prevents PATH environment pollution, eliminates unexpected system-wide side effects, and allows us to maintain a clean, auditable development environment.

### B. Installation via PowerShell
To install Scoop natively, open an elevated **PowerShell** console (search for PowerShell in the Start menu, right-click, and select "Run as Administrator" to configure execution policies, then run as a standard user) and execute the following commands:

```powershell
# Configure execution policy to allow remote script execution for the current user
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser

# Download and execute the official Scoop installation script
Invoke-RestMethod get.scoop.sh | Invoke-Expression

# Add the 'extras' bucket to expose WebAssembly and Vulkan development packages
scoop bucket add extras
```

---

## II. Installing the $Wat_{\text{safe}}$ and Wasmtime Toolchain

To compile and execute our hand-crafted, stack-only WebAssembly Text Format ($Wat_{\text{safe}}$) Stage 0 compiler ($C_0$) [1.12], we install the WebAssembly Binary Toolkit (WABT) and the standalone Wasmtime JIT runtime.

### A. Installing WABT and Wasmtime
Execute the following Scoop commands in your PowerShell console to install the tools natively:

```powershell
# Install WABT (WebAssembly Binary Toolkit) - includes wat2wasm and wasm2wat
scoop install wabt

# Install Wasmtime (Standalone WASI-compliant WebAssembly runtime)
scoop install wasmtime
```

### B. Verifying the Toolchain
Once the installation completes, verify that the executables are accessible in your environment's PATH:

```powershell
# Verify WABT assembler is active
wat2wasm --version

# Verify Wasmtime JIT compiler is active
wasmtime --version
```

Both commands should return their respective version strings (e.g., `wat2wasm version 1.0.32` and `wasmtime 7.0.0`) without errors.

---

## III. Installing the Vulkan and SPIR-V SDK

To compile parallel mathematical operations, coordinate-free logistics, and local sheaf cohomology checks ($H^1 = 0$) into our **SPIR-V compute shader targets** [1.14], we require the official Vulkan Software Development Kit (SDK).

### A. Installing Vulkan via Scoop
The Vulkan SDK contains critical tools—including the `glslc` (or `glslangValidator`) compiler, `spirv-val` (the SPIR-V bytecode verifier), and `spirv-dis` (the SPIR-V disassembler). Install it via Scoop:

```powershell
# Install the Vulkan SDK
scoop install vulkan
```

### B. Activating the SPIR-V Compiler
To allow Vulkan applications on Windows 11 to discover the validation layers provided by the SDK, run the registration script included in the installation directory:

```powershell
# Navigate to the Vulkan installation directory (adjust the version folder if necessary)
cd ~\scoop\apps\vulkan\current

# Execute the registration script as an Administrator
powershell .\install-vk-layers.ps1
```

Verify that the SPIR-V compiler is active by running:

```powershell
glslc --version
```

The command should output the compiler version, confirming that your environment is ready to compile GLSL shaders directly to the SPIR-V target.

---

## IV. The NASA-Style Hardware Performance Profiling Setup

Because virtualization hypervisors (such as WSL2 or remote CI/CD virtual machines) block or heavily virtualize the physical **Performance Monitoring Unit (PMU)**, we must profile CPU cycles, cache-miss rates, and memory access latency **natively on Windows 11 bare-metal** to feed the static Cost Functor ($\mathcal{K}$).

### A. Native Windows Performance Counters via PowerShell
PowerShell provides native cmdlets to query the Windows Performance Counter library in real-time. To list the available processor and memory counter sets, run:

```powershell
# List available performance counter sets on your PC
Get-Counter -ListSet * | Select-Object CounterSetName
```

To run an instant, real-time sample of total CPU processor time and available physical RAM, execute:

```powershell
# Query instantaneous CPU utilization and free memory
Get-Counter -Counter "\Processor(_Total)\% Processor Time", "\Memory\Available MBytes"
```

### B. Microarchitectural Profiling via Windows Performance Toolkit
To measure fine-grained microarchitectural metrics (such as L1/L2 instruction cache misses and branch mispredictions) during $QnIR_0$ execution, we install the **Windows Performance Toolkit (WPT)**, which includes `xperf` and `wpr`.
1.  **Installation:** Download and install the **Windows Assessment and Deployment Kit (ADK)** from the official Microsoft site. During the installation wizard, **uncheck all features except "Windows Performance Toolkit"** to keep the footprint minimal.
2.  **Enabling Stack Walking:** Open an elevated Command Prompt (`cmd.exe` as Administrator) and run the registry update command to allow `xperf` to capture call stacks on 64-bit Windows:
    ```cmd
    REG ADD "HKLM\System\CurrentControlSet\Control\Session Manager\Memory Management" /v DisablePagingExecutive /t REG_DWORD /d 1 /f
    ```
    *Note: A system reboot is required for this registry change to take effect.*
3.  **Capturing a Performance Trace:** To capture a highly precise CPU and thread sampling trace while running your WebAssembly binary, execute the following command in an Administrator console:
    ```cmd
    xperf -on PROC_THREAD+LOADER+PROFILE -stackwalk profile -buffersize 1024 -MaxFile 1024 -FileMode Circular
    ```
    Run your compiled Wasm binary via Wasmtime. Once execution is complete, stop the trace and save it to a file:
    ```cmd
    xperf -d C:\QnIR_Trace.etl
    ```
    You can open `QnIR_Trace.etl` in the graphical **Windows Performance Analyzer (WPA)** to analyze the exact thread lifetimes and cache-miss overheads of your execution blocks.

---

## V. Setting Up the Lightweight Editor

To write and edit your $Wat_{\text{safe}}$ S-expressions with zero-latency syntax highlighting, we configure a lightweight, portable text editor.

### A. Installing VS Code via Scoop
Install the portable, zero-bloat version of Visual Studio Code:

```powershell
scoop install vscode
```

### B. Installing Wasm/Wat Extensions
Open VS Code, navigate to the Extensions Marketplace (Ctrl+Shift+X), and install the following lightweight extension:
*   **WebAssembly** (by *dany74q*): Provides complete syntax highlighting, folding, and autocomplete for WebAssembly Text Format (`.wat`) files.

---

## VI. Verifying the Stage 0 Bootstrapping Loop

We verify the setup by assembling, executing, and profiling a minimal, strictly typed, and stack-only $Wat_{\text{safe}}$ program.

### A. Writing the Stage 0 Seed Program
Create a new file named `C0_seed.wat` in your workspace and paste the following safe, memory-free Wat code:

```wat
(module
  ;; Import the WASI console write function
  (import "wasi_snapshot_preview1" "proc_exit" (func $proc_exit (param i32)))

  ;; Define a safe, stack-only 2-adic addition function (add2)
  (func $add2 (param $x i64) (param $y i64) (result i64)
    local.get $x
    local.get $y
    i64.add
  )

  ;; Main entry point of the Stage 0 seed compiler
  (func (export "_start")
    ;; Push operands onto the typed stack (calculating 42 + 13)
    i64.const 42
    i64.const 13
    call $add2

    ;; If the result is 55, exit successfully (status 0)
    i64.const 55
    i64.eq
    if
      i32.const 0
      call $proc_exit
    else
      i32.const 1
      call $proc_exit
    end
  )
)
```

### B. Assembling, Executing, and Profiling
To verify the compilation pipeline, execute the following commands in your PowerShell console:

```powershell
# 1. Assemble the Wat text file into binary Wasm
wat2wasm C0_seed.wat -o C0_seed.wasm

# 2. Execute the Wasm binary securely inside Wasmtime WASI sandbox
wasmtime run C0_seed.wasm

# 3. Query the host execution telemetry to verify CPU cycle overhead
Get-Counter -Counter "\Processor(_Total)\% Processor Time" -SampleInterval 1 -MaxSamples 3
```

If the execution completes silently and returns zero errors, your environment is successfully configured. The $Wat_{\text{safe}}$ compilation pipeline is ready, and your local Windows 11 PC is optimized to begin the co-design of **Series 02 (QnIR)**.
