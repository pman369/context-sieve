# The Non-Technical Guide to context-sieve
### A Step-by-Step Tutorial for Everyone (Windows, Mac & Linux)

---

## 1. What is context-sieve? (In Plain English)

When you write code and ask an AI assistant (like ChatGPT, Claude, GitHub Copilot, or Cursor) for help, your editor quietly packages up your code and sends it across the internet to the AI.

### The Problem
Most code files are filled with **distractions**:
* Old functions that aren't being used.
* Huge paragraphs of obsolete comments.
* Helper tools that have nothing to do with what you are currently working on.

When you send all that "clutter" to an AI, the AI gets overwhelmed and confused. It often invents code that doesn't exist, references outdated tools, or takes a long time to answer.

### The Solution: context-sieve
Think of `context-sieve` as a **smart highlighter**:
1. It sits quietly on your computer in the background.
2. It looks at where your mouse cursor is right now.
3. It figures out *only* the specific pieces of code you are actually touching.
4. It temporarily sweeps away the unrelated dead code (often **shrinking the data by 50% to 70%**) before handing it to the AI.
5. **Crucial rule**: It **never** changes or damages your real files on disk. Your actual code stays 100% safe and intact.

---

## 2. Which File Do I Download?

Head to the official releases page:
👉 **[https://github.com/pman369/context-sieve/releases](https://github.com/pman369/context-sieve/releases)**

Look at your computer type in the table below and download that exact file:

| Your Computer | What to Download |
|---|---|
| **Windows PC** (Windows 10 or 11) | `context-sieve-v0.1.1-x86_64-pc-windows-msvc.zip` |
| **Mac with Apple Silicon** (M1, M2, M3, M4 chips) | `context-sieve-v0.1.1-aarch64-apple-darwin.tar.gz` |
| **Older Mac** (with Intel processor) | `context-sieve-v0.1.1-x86_64-apple-darwin.tar.gz` |
| **Linux PC** (Ubuntu, Debian, Fedora, Mint) | `context-sieve-v0.1.1-x86_64-universal-linux-gnu.tar.gz` |
| **Raspberry Pi / ARM64 Linux** | `context-sieve-v0.1.1-aarch64-universal-linux-gnu.tar.gz` |
| **Web Developers / In-Browser Apps** | `context-sieve-v0.1.1-wasm.tar.gz` |

---

## 3. Step-by-Step Instructions by Operating System

---

### 🪟 Windows (Step-by-Step)

#### Step 1: Extract the Downloaded Folder
1. Open your **Downloads** folder.
2. Find `context-sieve-v0.1.1-x86_64-pc-windows-msvc.zip`.
3. **Right-click** on the zip file and click **Extract All...**, then click **Extract**.
4. You will now see a folder with two files inside:
   * `context-sieve-daemon.exe` (the program)
   * `README.md` (the technical manual)

#### Step 2: Open the Welcome Screen
* Simply **double-click** `context-sieve-daemon.exe`.
* A black command window will open with a welcome screen explaining the program and showing you available options.
* It will say `Press Enter to exit...`. Pressing `Enter` closes the window cleanly.

> **Note on Windows Security ("Windows protected your PC")**:
> If Windows SmartScreen pops up saying *"Windows protected your PC"*, click **More info** and then click **Run anyway**. This appears because the software is freshly compiled and not purchased through the Microsoft Store.

#### Step 3: Test Pruning a File in 10 Seconds
Want to see the smart highlighter in action without setting up anything complex?
1. Open the folder where you extracted `context-sieve-daemon.exe`.
2. Click on the folder's address bar at the top, type `powershell`, and press `Enter`. A PowerShell window will open directly in that folder.
3. Type (or paste) this command to test it on any code file you have:
   ```powershell
   .\context-sieve-daemon.exe --file "C:\path\to\your\code.rs"
   ```
   *(You can test `.rs` Rust files, `.py` Python files, `.ts` TypeScript files, or `.go` Go files).*
4. You will immediately see a report showing:
   * How many bytes your original code was.
   * How much dead code was removed (e.g. `Reduction: 55.6%`).
   * A preview of the clean, condensed code ready for the AI!

#### Step 4: Run as a Background Service for Your Editor
To have it run continuously in the background and filter code for your editor:
```powershell
.\context-sieve-daemon.exe --upstream rust-analyzer
```
*(If you are writing Python, replace `rust-analyzer` with `pyright` or `pylsp`).*

You will see:
```text
Context Sieve Daemon v0.1.1
Listening on: \\.\pipe\context-sieve
Upstream LSP: rust-analyzer
Waiting for editor connections... (Press Ctrl+C to stop)
```
Leave this window open while you work. When you are done, press `Ctrl + C` to stop it.

---

### 🍎 Mac (macOS Step-by-Step)

#### Step 1: Extract the Download
1. Open your **Downloads** folder.
2. Double-click the downloaded `.tar.gz` file (e.g. `context-sieve-v0.1.1-aarch64-apple-darwin.tar.gz`).
3. macOS will automatically extract it into a folder containing `context-sieve-daemon` and `README.md`.

#### Step 2: Allow the App Through Gatekeeper
Because the binary is downloaded from GitHub, macOS may show a warning: *"cannot be opened because it is from an unidentified developer"*.
* **Easy fix**:
  1. Open your Mac's **System Settings** (or **System Preferences**).
  2. Click **Privacy & Security**.
  3. Scroll down to the **Security** section.
  4. You will see a note saying *"context-sieve-daemon was blocked"*. Click **Open Anyway** and enter your password.

#### Step 3: Test in the Terminal
1. Open your Mac's **Terminal** app (Press `Command + Space`, type `Terminal`, and press `Enter`).
2. Move into the extracted folder:
   ```bash
   cd ~/Downloads/context-sieve-*
   ```
3. Make sure the file has permission to run:
   ```bash
   chmod +x context-sieve-daemon
   ```
4. Run the welcome guide:
   ```bash
   ./context-sieve-daemon
   ```
5. Test pruning any source file:
   ```bash
   ./context-sieve-daemon --file /path/to/my_script.py
   ```

#### Step 4: Run as a Background Service
To run it for your code editor:
```bash
./context-sieve-daemon --upstream rust-analyzer
```
It will start listening on `/tmp/context-sieve.sock` and wait for your editor to connect.

---

### 🐧 Linux (Step-by-Step)

#### Step 1: Extract the Archive
Open your terminal and run:
```bash
cd ~/Downloads
tar -xzf context-sieve-v0.1.1-x86_64-universal-linux-gnu.tar.gz
cd context-sieve-v0.1.1-*
chmod +x context-sieve-daemon
```

#### Step 2: Quick Test on a Code File
```bash
./context-sieve-daemon --file ~/my_project/src/main.rs --depth 2
```

#### Step 3: Run as Background Daemon
```bash
./context-sieve-daemon --upstream rust-analyzer
```
The daemon creates a Unix socket at `/tmp/context-sieve.sock`.

---

## 4. How to Connect It to Your Code Editor

`context-sieve` acts as a middleman between your editor and your programming tools. Here is how you connect it:

### Visual Studio Code / Cursor
If your editor uses an extension for your language (like `rust-analyzer` or `Python`):
1. Open your editor's **Settings** (`Ctrl + ,` on Windows/Linux or `Cmd + ,` on Mac).
2. Search for the server path setting for your language:
   * **Windows**: Set the connection pipe path to `\\.\pipe\context-sieve`.
   * **Mac & Linux**: Set the connection socket path to `/tmp/context-sieve.sock`.
3. Now, whenever your editor sends code changes, `context-sieve` intercepts the message, prunes the dead code in under 1.5 milliseconds, and forwards clean context!

### Neovim
In your Neovim configuration (e.g. `init.lua`), configure your LSP client to attach to the socket/pipe:
```lua
-- For Linux / macOS:
local pipe = vim.loop.new_pipe(false)
vim.loop.pipe_connect(pipe, "/tmp/context-sieve.sock", function(err)
    -- Connected to context-sieve
end)
```

---

## 5. Frequently Asked Questions (FAQ)

### Q1: I double-clicked the file on Windows and a black window opened. Is it broken?
**No, it is working completely normally!**
`context-sieve` is a background tool (called a "daemon" or "service"). It is not a visual game or word processor with buttons. When you double-click it, it shows you instructions and waits for your confirmation.

### Q2: Can `context-sieve` accidentally delete my code?
**Never.**
`context-sieve` works strictly in computer memory. It takes a *copy* of your text, trims out the dead parts, and hands that trimmed copy to the AI. Your actual files saved on your hard drive are **never** modified or deleted.

### Q3: How do I turn it off or stop it?
Whenever `context-sieve` is running in a terminal or command prompt window:
* Simply press **`Ctrl + C`** on your keyboard, OR
* Close the terminal window by clicking the `X` button.

### Q4: What does `--depth 2` mean?
`depth` controls how far `context-sieve` looks for connected code:
* **Depth 1**: Only keeps the function your cursor is directly inside.
* **Depth 2 (Default & Recommended)**: Keeps the function your cursor is inside, plus any helper functions that function directly calls.
* **Depth 3+**: Keeps code further down the call chain.

### Q5: What programming languages are supported?
`context-sieve` currently includes built-in support for:
* **Rust** (`.rs`)
* **Python** (`.py`)
* **TypeScript / TSX** (`.ts`, `.tsx`)
* **Go** (`.go`)
More languages are continually added.

---

## 6. Quick Cheat Sheet

| I want to... | Windows Command | Mac / Linux Command |
|---|---|---|
| **View help & options** | `.\context-sieve-daemon.exe --help` | `./context-sieve-daemon --help` |
| **Test prune a single file** | `.\context-sieve-daemon.exe --file file.py` | `./context-sieve-daemon --file file.py` |
| **Run background daemon (Rust)** | `.\context-sieve-daemon.exe -u rust-analyzer` | `./context-sieve-daemon -u rust-analyzer` |
| **Run background daemon (Python)** | `.\context-sieve-daemon.exe -u pyright` | `./context-sieve-daemon -u pyright` |
| **Stop the program** | Press `Ctrl + C` | Press `Ctrl + C` |
