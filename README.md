# 🐕 RustHound: Gerçek Zamanlı Log Analiz ve İzleme Aracı

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()
[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)]()

**RustHound**, Rust ile geliştirilmiş yüksek performanslı, açık kaynaklı gerçek zamanlı log analiz ve izleme aracıdır. Sistem yöneticileri, DevOps mühendisleri ve geliştiriciler için tasarlanmış bu araç, log verilerinizi sürekli olarak izleyerek kritik olayları anında tespit eder.

## 🎯 Neden RustHound?

- **⚡ Yüksek Performans**: Rust'ın bellek güvenliği ve hızından yararlanır
- **🔍 Akıllı Analiz**: Regex ve pattern matching ile gelişmiş log analizi
- **📊 Gerçek Zamanlı İzleme**: Canlı log takibi ve anında uyarılar
- **🛠️ Kolay Yapılandırma**: TOML tabanlı basit konfigürasyon sistemi
- **🌐 Çapraz Platform**: Linux, macOS ve Windows desteği

## İçindekiler

- [Özellikler](#-özellikler)
- [Kurulum](#-kurulum)
- [Kullanım](#-kullanım)
- [Yapılandırma](#-yapılandırma)
- [Katkıda Bulunma](#-katkıda-bulunma)
- [Lisans](#-lisans)

## ✨ Özellikler

*   **Gerçek Zamanlı Log İzleme:** Belirtilen log dosyalarını gerçek zamanlı olarak izler ve yeni girişleri anında işler.
*   **Kural Tabanlı Desen Eşleştirme:** Yapılandırılabilir kurallar (`rules.toml`) kullanarak log girişlerinde belirli metin desenlerini veya regex ifadelerini eşleştirir.
*   **Frekans Analizi:** Tanımlanan desenlerin belirli zaman aralıklarında ne sıklıkta ortaya çıktığını izler ve eşik değerleri aşıldığında uyarı verir.
*   **Esnek Çıktı Seçenekleri:** Analiz sonuçlarını konsola yazdırabilir veya yapılandırılabilir JSON dosyalarına kaydedebilir.
*   **Çapraz Platform Uyumluluğu:** Linux, macOS ve Windows'ta sorunsuz çalışır.

## 🚀 Kurulum

### Sistem Gereksinimleri

- **Rust 1.70+** - [rustup.rs](https://rustup.rs/) üzerinden yükleyebilirsiniz
- **Git** (kaynak koddan kurulum için)
- **Minimum 50MB disk alanı**

### Hızlı Kurulum

#### Cargo ile (Önerilen)
```bash
# Crates.io'dan direkt kurulum (yakında)
cargo install rust_hound
```

#### Kaynak Koddan Kurulum
```bash
# 1. Depoyu klonlayın
git clone https://github.com/rustfuture/RustHound.git
cd RustHound

# 2. Bağımlılıkları yükleyin ve derleyin
cargo build --release

# 3. Sistem genelinde kullanım için yükleyin
cargo install --path .
```

#### Doğrudan İkili Dosya İndirme
```bash
# GitHub Releases'den en son sürümü indirin
wget https://github.com/rustfuture/RustHound/releases/latest/download/rust_hound-linux-x64.tar.gz
tar -xzf rust_hound-linux-x64.tar.gz
sudo mv rust_hound /usr/local/bin/
```

### Kurulum Doğrulama
```bash
rust_hound --version
rust_hound --help
```

## 📖 Kullanım

### Hızlı Başlangıç

```bash
# Basit log dosyası analizi
rust_hound --file /var/log/app.log

# Dizin içindeki tüm log dosyalarını analiz et
rust_hound --dir /var/log/

# Gerçek zamanlı izleme (tail -f benzeri)
rust_hound --file /var/log/app.log --follow
```

### Komut Satırı Referansı

| Parametre | Kısaltma | Açıklama | Varsayılan |
|-----------|----------|----------|------------|
| `--file <PATH>` | `-f` | Tek bir log dosyasının yolu | `sample.log` |
| `--dir <PATH>` | `-d` | Log dosyalarını içeren dizin | `.` |
| `--rules <PATH>` | `-r` | Kurallar dosyasının yolu | `rules.toml` |
| `--output <FORMAT>` | `-o` | Çıktı formatı (`console`, `json`, `both`) | `console` |
| `--follow` | `-F` | Gerçek zamanlı izleme modu | `false` |
| `--severity <LEVEL>` | `-s` | Minimum önem seviyesi | - |
| `--verbose` | `-v` | Detaylı çıktı | `false` |
| `--help` | `-h` | Yardım bilgilerini göster | - |
| `--version` | `-V` | Sürüm bilgilerini göster | - |

### Kullanım Örnekleri

#### Temel Log Analizi
```bash
# Tek dosya analizi
rust_hound -f /var/log/nginx/access.log

# Özel kurallar ile analiz
rust_hound -f app.log -r custom_rules.toml
```

#### Gerçek Zamanlı İzleme
```bash
# Canlı log takibi
rust_hound -f /var/log/syslog --follow

# JSON çıktısı ile canlı takip
rust_hound -d /var/log/ --follow -o json
```

#### Filtreleme ve Çıktı
```bash
# Sadece kritik seviye uyarılar
rust_hound -f app.log -s critical

# Hem konsol hem JSON çıktısı
rust_hound -f app.log -o both

# Detaylı hata ayıklama
rust_hound -f app.log --verbose
```

#### Toplu İşleme
```bash
# Tüm log dizinini analiz et
rust_hound -d /var/log/ -r production_rules.toml

# Belirli pattern'ler için tarama
rust_hound -d /home/user/logs/ -s high -o json
```

## ⚙️ Yapılandırma

RustHound, esnek TOML tabanlı yapılandırma sistemi kullanır. Varsayılan olarak `rules.toml` dosyasını arar.

### Temel Yapılandırma Dosyası (`rules.toml`)

```toml
[general]
# Genel ayarlar
case_sensitive = false
max_line_length = 1024

[patterns]
# Basit string pattern'ler
error_keywords = ["ERROR", "FATAL", "CRITICAL"]
warning_keywords = ["WARN", "WARNING", "DEPRECATED"]

[regex_rules]
# Gelişmiş regex kuralları
[[regex_rules.rule]]
name = "SQL Injection Attempt"
pattern = "(?i)(union|select|insert|delete|drop).*['\";]"
severity = "critical"
description = "Potansiyel SQL injection saldırısı tespit edildi"

[[regex_rules.rule]]
name = "Failed Login"
pattern = "(?i)failed.*login|authentication.*failed"
severity = "high"
description = "Başarısız giriş denemesi"

[frequency_rules]
# Frekans tabanlı kurallar
[[frequency_rules.rule]]
name = "Too Many Errors"
pattern = "ERROR"
threshold = 10
window_seconds = 60
severity = "critical"
description = "1 dakika içinde çok fazla hata"

[output]
# Çıktı ayarları
timestamp_format = "%Y-%m-%d %H:%M:%S"
include_line_numbers = true
color_output = true
```

### Yapılandırma Seçenekleri

#### Genel Ayarlar
- `case_sensitive`: Büyük/küçük harf duyarlılığı
- `max_line_length`: Maksimum satır uzunluğu
- `ignore_empty_lines`: Boş satırları yoksay

#### Pattern Türleri
- **Basit String Pattern'ler**: Hızlı anahtar kelime eşleştirme
- **Regex Pattern'ler**: Karmaşık desen eşleştirme
- **Frekans Kuralları**: Zaman tabanlı eşik kontrolü

#### Önem Seviyeleri
- `critical`: Kritik durumlar
- `high`: Yüksek öncelik
- `medium`: Orta öncelik  
- `low`: Düşük öncelik
- `info`: Bilgilendirme

### Örnek Yapılandırmalar

#### Web Sunucu Logları
```toml
[[regex_rules.rule]]
name = "HTTP 5xx Errors"
pattern = "HTTP/1\.[01]\" [5][0-9][0-9]"
severity = "high"

[[regex_rules.rule]]
name = "Slow Response"
pattern = "response_time:[0-9]{4,}"
severity = "medium"
```

#### Uygulama Logları
```toml
[[regex_rules.rule]]
name = "Memory Leak Warning"
pattern = "(?i)memory.*leak|out.*of.*memory"
severity = "critical"

[[regex_rules.rule]]
name = "Database Connection Error"
pattern = "(?i)database.*connection.*failed|db.*timeout"
severity = "high"
```

## 🤝 Katkıda Bulunma

RustHound açık kaynak bir projedir ve katkılarınızı memnuniyetle karşılarız!

### Nasıl Katkıda Bulunabilirsiniz?

1. **🐛 Bug Raporları**: [Issues](https://github.com/rustfuture/RustHound/issues) sayfasından bug bildirin
2. **💡 Özellik İstekleri**: Yeni özellik önerilerinizi paylaşın
3. **📝 Dokümantasyon**: README, kod yorumları ve örnekleri iyileştirin
4. **🔧 Kod Katkıları**: Pull request gönderin

### Geliştirme Ortamı Kurulumu

```bash
# Projeyi fork edin ve klonlayın
git clone https://github.com/rustfuture/RustHound.git
cd RustHound

# Geliştirme bağımlılıklarını yükleyin
cargo build

# Testleri çalıştırın
cargo test

# Kod formatını kontrol edin
cargo fmt --check
cargo clippy
```

### Katkı Kuralları

- Kod değişikliklerinden önce issue açın
- Commit mesajlarında [Conventional Commits](https://conventionalcommits.org/) kullanın
- Yeni özellikler için test yazın
- Dokümantasyonu güncel tutun

## 📊 Performans

- **Bellek Kullanımı**: ~10-50MB (dosya boyutuna bağlı)
- **İşleme Hızı**: ~100K satır/saniye
- **Desteklenen Dosya Boyutu**: Sınırsız (streaming işleme)
- **Eş Zamanlı Dosya**: 100+ dosya

## 🔧 Sorun Giderme

### Yaygın Sorunlar

**Problem**: `Permission denied` hatası
```bash
# Çözüm: Dosya izinlerini kontrol edin
chmod +r /var/log/app.log
```

**Problem**: Yüksek bellek kullanımı
```bash
# Çözüm: Streaming mode kullanın
rust_hound -f large_file.log --follow
```

**Problem**: Regex pattern çalışmıyor
```bash
# Çözüm: Pattern'i test edin
rust_hound -f test.log --verbose
```

## 📜 Lisans

Bu proje **Apache License 2.0** altında lisanslanmıştır.

```
Copyright 2024 RustHound Contributors

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```

Detaylar için [LICENSE](LICENSE) dosyasına bakın.

---

<div align="center">

**⭐ Projeyi beğendiyseniz yıldız vermeyi unutmayın!**

[🐛 Bug Bildir](https://github.com/rustfuture/RustHound/issues) • [💡 Özellik İste](https://github.com/rustfuture/RustHound/issues) • [📖 Dokümantasyon](https://github.com/rustfuture/RustHound/wiki)

</div>