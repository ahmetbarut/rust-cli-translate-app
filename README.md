# translate-cli

## Overview

Translate-cli is a simple command-line application for translating texts. This document covers installation and usage instructions.

## Installation

1. Download the installer by running the following command:  
   ```bash
   curl -O https://github.com/ahmetbarut/rust-cli-translate-app/releases/latest/download/translate-cli
   ```
2. Make the file executable:  
   ```bash
   chmod +x translate-cli
   ```
3. Move the file to your personal bin directory (it will install in your home directory, no sudo required):  
   ```bash
   mv translate-cli $HOME/bin
   ```
4. Ensure your $HOME/bin is in your PATH environment variable.

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
