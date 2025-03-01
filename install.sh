#!/usr/bin/env bash
set -e

# Uygulama versiyonu ve indirme URL'si (kendi GitHub deposunuza göre düzenleyin)
VERSION="latest"
URL="https://github.com/ahmetbarut/rust-cli-translate-app/releases/${VERSION}/download/translate-cli"

# Kurulum dizinini kullanıcının ev dizinine ayarla
INSTALL_DIR="$HOME/bin"

# Hedef dizin yoksa oluştur
mkdir -p "$INSTALL_DIR"

echo "Downloading translate-cli version ${VERSION}..."
curl -L "$URL" -o translate-cli
chmod +x translate-cli

# Build dosyasını çekme
BUILD_URL="http://example.com/path/to/build-artifact"  # Build çıktısının URL'si
echo "Build dosyası $BUILD_URL adresinden indiriliyor..."
curl -L "$BUILD_URL" -o translate-cli || { echo "Dosya indirilemedi."; exit 1; }
chmod +x translate-cli

echo "Installing translate-cli to ${INSTALL_DIR}..."
# sudo kaldırıldı
mv translate-cli "$INSTALL_DIR"

echo "Installation completed! You can now use 'translate-cli'."
