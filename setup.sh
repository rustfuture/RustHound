#!/bin/bash

INSTALL_DIR="$HOME/.local/bin"
CONFIG_DIR="$HOME/.config/rusthound"

install_rusthound() {
    echo "RustHound Kurulumu Başlatılıyor..."

    # Rust'ın kurulu olup olmadığını kontrol et
    if ! command -v rustc &> /dev/null
    then
        echo "Rust bulunamadı. Lütfen https://rustup.rs/ adresinden Rust'ı yükleyin."
        exit 1
    fi

    echo "Rust algılandı. Proje derleniyor..."

    # Projeyi derle
    cargo build --release
    if [ $? -ne 0 ]; then
        echo "Derleme başarısız oldu. Lütfen hataları kontrol edin."
        exit 1
    fi

    echo "Derleme tamamlandı. Yürütülebilir dosya kopyalanıyor..."

    # Yürütülebilir dosyayı ~/.local/bin dizinine kopyala
    mkdir -p "$INSTALL_DIR"
    cp target/release/rusthound "$INSTALL_DIR/"

    if [ $? -ne 0 ]; then
        echo "Yürütülebilir dosya kopyalanamadı. İzinleri kontrol edin veya manuel olarak kopyalayın."
        exit 1
    fi

    echo "RustHound başarıyla $INSTALL_DIR dizinine kuruldu."

    # rules.toml dosyasını ~/.config/rusthound dizinine kopyala
    mkdir -p "$CONFIG_DIR"
    cp rules.toml "$CONFIG_DIR/rules.toml"

    if [ $? -ne 0 ]; then
        echo "rules.toml dosyası kopyalanamadı. Lütfen manuel olarak kopyalayın: cp rules.toml $CONFIG_DIR/rules.toml"
    fi

    echo ""
    echo "Sonraki Adımlar:"
    echo "1. Varsayılan 'rules.toml' dosyası $CONFIG_DIR/rules.toml konumuna kopyalandı. Bu dosyayı kendi ihtiyaçlarınıza göre düzenleyebilirsiniz."
    echo "2. '$INSTALL_DIR' dizininin PATH değişkeninizde olduğundan emin olun. Değilse, aşağıdaki komutu .bashrc veya .zshrc dosyanıza ekleyin:"
    echo "   export PATH=\"$PATH:$INSTALL_DIR\""
    echo "   Ardından 'source ~/.bashrc' veya 'source ~/.zshrc' komutunu çalıştırın."
    echo ""
    echo "Kullanım: rusthound /path/to/your/logfile.log"
    echo "Kurulum tamamlandı!"
}

uninstall_rusthound() {
    echo "RustHound Kaldırma İşlemi Başlatılıyor..."

    if [ -f "$INSTALL_DIR/rusthound" ]; then
        rm "$INSTALL_DIR/rusthound"
        echo "RustHound yürütülebilir dosyası kaldırıldı: $INSTALL_DIR/rusthound"
    else
        echo "RustHound yürütülebilir dosyası bulunamadı: $INSTALL_DIR/rusthound"
    fi

    if [ -d "$CONFIG_DIR" ]; then
        rm -rf "$CONFIG_DIR"
        echo "RustHound yapılandırma dizini kaldırıldı: $CONFIG_DIR"
    else
        echo "RustHound yapılandırma dizini bulunamadı: $CONFIG_DIR"
    fi

    echo ""
    echo "Kaldırma işlemi tamamlandı."
    echo "Not: Eğer PATH değişkeninize $INSTALL_DIR dizinini eklediyseniz, bu değişikliği .bashrc veya .zshrc gibi kabuk yapılandırma dosyanızdan manuel olarak kaldırmanız gerekebilir."
}

# Ana menü
clear
echo "RustHound Kurulum/Kaldırma Menüsü"
echo "----------------------------------"
echo "1. RustHound'u Kur"
echo "2. RustHound'u Kaldır"
echo "3. Çıkış"
echo "----------------------------------"

read -p "Seçiminizi yapın (1-3): " choice

case $choice in
    1)
        install_rusthound
        ;;
    2)
        uninstall_rusthound
        ;;
    3)
        echo "Çıkılıyor."
        ;;
    *)
        echo "Geçersiz seçim. Lütfen 1, 2 veya 3 girin."
        ;;
esac
