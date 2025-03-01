# translate-cli

## Overview

Translate-cli is a simple command-line application for translating texts. This document covers installation and usage instructions.

## Installation

### Option 1: Using the Installer Script

The easiest way to install is using our installer script:

```bash
curl -sSL https://raw.githubusercontent.com/ahmetbarut/rust-cli-translate-app/main/install.sh | bash
```

This script will automatically download the latest version, make it executable, and place it in your `$HOME/bin` directory.

### Option 2: Manual Installation

If you prefer to install manually:

1. Download the latest release:  
    ```bash
    curl -O https://github.com/ahmetbarut/rust-cli-translate-app/releases/latest/download/translate-cli
    ```
2. Make the file executable:  
    ```bash
    chmod +x translate-cli
    ```
3. Move the file to your personal bin directory:  
    ```bash
    mv translate-cli $HOME/bin
    ```
4. Ensure your `$HOME/bin` is in your PATH environment variable.

## Usage

To translate a text, simply run:  
```bash
translate-cli 'Your text to translate'
```
This will output the translated text. Please refer to the help command for more options:  
```bash
translate-cli --help
```

## Troubleshooting

- If you encounter permission errors, make sure $HOME/bin is writable and added to your PATH.  
- For further issues, consult the official repository documentation or open an issue on GitHub.

## Additional Information

Refer to the project repository for updates and additional options, features, or bug fixes.

## Installation and Usage

### DeepL Free Version Support and API Key Setup

This application works with the free version of DeepL. You will need a DeepL API key to use the application.

After obtaining your API key, you can set it up according to your shell by following these steps:

- **Bash:** Add the following line to your `~/.bashrc` file:  
    `export DEEPL_API_KEY="your_api_key"`
    
- **Zsh:** Add the following line to your `~/.zshrc` file:  
    `export DEEPL_API_KEY="your_api_key"`
    
- **Fish:** If you use Fish shell, add this line to your `config.fish` file:  
    `set -x DEEPL_API_KEY your_api_key`

After making changes, restart your terminal or source the relevant file. For example, for bash you can use the command `source ~/.bashrc`.

Also, please review the necessary settings in `install.sh`.
