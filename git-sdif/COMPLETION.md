# Git-sdif Bash Completion

This package contains bash completion support for the `git-sdif` command, providing the same branch auto-completion experience as native git commands.

## Features

- **Branch name completion** for both parameters of `git-sdif`
- **Prefix filtering** - type partial branch names and press TAB to filter results
- **Git-compatible behavior** - uses the same completion functions as git
- **Supports all git refs** - branches, tags, HEAD, ORIG_HEAD, etc.
- **No completion beyond second parameter** - prevents unwanted completions

## Installation

### Automatic Installation

Run the installation script from the git-sdif directory:

```bash
./install.sh
```

Or from the workspace root:

```bash
./git-sdif/install.sh
```

This will:
1. Build and install the `git-sdif` binary to `~/.local/bin/`
2. Automatically install the completion script to `~/.bash_completion.d/`
3. Provide setup instructions for your shell

### Manual Installation

1. Copy the completion script:
   ```bash
   mkdir -p ~/.bash_completion.d
   cp git-sdif-completion.bash ~/.bash_completion.d/
   ```

2. Add to your `~/.bashrc`:
   ```bash
   # Load custom bash completions
   if [ -d ~/.bash_completion.d ]; then
       for file in ~/.bash_completion.d/*; do
           [ -r "$file" ] && source "$file"
       done
   fi
   ```

3. Reload your shell or source the completion:
   ```bash
   source ~/.bash_completion.d/git-sdif-completion.bash
   ```

## Build Integration

The completion script is automatically installed when building git-sdif in release mode from the workspace root:

```bash
cargo build --release -p git-sdif
```

This uses the build script (`build.rs`) to automatically copy the completion script to `~/.bash_completion.d/` during compilation.

## Usage Examples

Once installed, you can use TAB completion with both `git-sdif` and `git sdif`:

```bash
# Complete all available branches
git-sdif <TAB>
git sdif <TAB>
# Shows: HEAD  ORIG_HEAD  master  develop  feature-branch  origin/master  ...

# Complete branches starting with 'm'
git-sdif m<TAB>
git sdif m<TAB>
# Shows: master  main  my-feature  ...

# Complete second parameter
git-sdif master <TAB>
git sdif master <TAB>
# Shows: HEAD  ORIG_HEAD  master  develop  feature-branch  origin/master  ...

# Complete second parameter with prefix
git-sdif master dev<TAB>
git sdif master dev<TAB>
# Shows: develop  development  ...
```

## How It Works

The completion script:

1. **Integrates with git's completion system** - uses `__git_complete` for `git sdif` subcommand
2. **Handles both parameters** - provides completion for `branch1` and `branch2`
3. **Filters by prefix** - only shows branches matching what you've typed
4. **Stops at two parameters** - no completion for additional arguments
5. **Works for both forms** - `git-sdif` standalone and `git sdif` subcommand
6. **Uses git references** - includes branches, tags, and special refs like HEAD

## Testing

To test the completion manually:

1. Make sure you're in a git repository with multiple branches
2. Source the completion script:
   ```bash
   source ~/.bash_completion.d/git-sdif-completion.bash
   ```
3. Test completion:
   ```bash
   git sdif <TAB>
   git-sdif <TAB>
   ```

## Troubleshooting

### Completion not working

1. Make sure the completion is loaded:
   ```bash
   source ~/.bash_completion.d/git-sdif-completion.bash
   ```

2. Verify the function is defined:
   ```bash
   declare -f _git_sdif
   ```

3. Check if completion is registered:
   ```bash
   complete -p git-sdif
   complete -p | grep sdif
   ```

### No branch names shown

1. Make sure you're in a git repository
2. Verify git completion is available:
   ```bash
   declare -f __git_complete
   ```

3. Test git's own completion:
   ```bash
   git checkout <TAB>
   git branch <TAB>
   ```

## Compatibility

- **Bash 3.2+** - Compatible with older bash versions
- **Git completion** - Works with standard git completion scripts
- **Multiple git installations** - Adapts to different git completion locations
- **Fallback mode** - Basic completion even without full git completion support

## Files

- `git-sdif-completion.bash` - Main completion script
- `build.rs` - Build script that auto-installs completion
- `install.sh` - Complete installation script (binary + completion)
- `COMPLETION.md` - This documentation file
