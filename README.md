# RustHound: Gerçek Zamanlı Log Analiz ve İzleme Aracı

![RustHound Banner](https://your-image-url.com/banner.png) <!-- Eğer varsa, gerçek bir banner URL'si ile değiştirin -->

**RustHound, Rust ile titizlikle hazırlanmış güçlü, açık kaynaklı, gerçek zamanlı bir log analiz ve izleme aracıdır.** Sistem yöneticileri, geliştiriciler ve log verilerini sürekli olarak izlemesi gereken herkes için tasarlanmıştır.

Log dosyalarınızdaki kritik olayları, desenleri ve anormallikleri manuel olarak arama zahmetine veda edin. RustHound ile loglarınızı tanımladığınız kurallara göre otomatik olarak izleyebilir ve anında değerli bilgiler edinebilirsiniz!

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

### Ön Koşullar

**Rust**'ın kurulu olduğundan emin olun. `rustup` aracılığıyla [https://rustup.rs/](https://rustup.rs/) adresinden yükleyebilirsiniz.

### Crates.io'dan (Önerilen)

*Yakında! Yayınlandıktan sonra şu komutla yükleyebilirsiniz:*
```bash
cargo install rust_hound
```

### Kaynaktan

1.  **Depoyu Klonlayın:**
    ```bash
    git clone https://github.com/your-username/RustHound.git # Depo URL'nizi güncelleyin
    cd RustHound
    ```

2.  **`cargo` kullanarak Kurun:**
    Bu komut, ikili dosyayı derleyecek ve cargo bin yolunuza (`~/.cargo/bin/`) yerleştirecek, böylece sistem genelinde erişilebilir olacaktır.
    ```bash
cargo install --path .
    ```

## Kullanım

Kurulduktan sonra, `rust_hound` komutunu doğrudan terminalinizden kullanabilirsiniz.

### Temel Kullanım

```bash
# Tek bir dosyayı analiz et
rust_hound --file /yol/to/logdosyanız.log

# Bir dizindeki tüm .log dosyalarını analiz et
rust_hound --dir /yol/to/loglarınız/
```

### Komut Satırı Argümanları

| Bayrak | Takma Ad | Açıklama | Varsayılan |
| :--- | :--- | :--- | :--- |
| `--file <DOSYA>` | `-f` | Tek bir log dosyasının yolu. | `sample.log` |
| `--dir <DİZİN>` | `-d` | .log dosyalarını içeren bir dizinin yolu. | `.` |
| `--rules <KURALLAR>` | `-r` | Kurallar dosyasının yolu. | `rules.toml` |
| `--output <ÇIKTI>` | `-o` | Çıktı formatı: `console`, `json`, `both`. | `console` |
| `--follow` | `-F` | `tail -f` modunu etkinleştir (gerçek zamanlı izleme). | `false` |
| `--severity <ÖNEM>` | `-s` | Önem seviyesine göre filtrele: `critical`, `high`, vb. | |
| `--verbose` | `-v` | Hata ayıklama çıktısını etkinleştir. | `false` |
| `--help` | `-h` | Yardım bilgilerini yazdır. | |
| `--version` | `-V` | Sürüm bilgilerini yazdır. | |

**Örnek:** Bir dizini gerçek zamanlı olarak izle ve JSON çıktısı al:
```bash
rust_hound --dir /var/log/ --follow --output json
```

## ⚙️ Yapılandırma

RustHound'un davranışı `rules.toml` dosyası aracılığıyla özelleştirilebilir. Varsayılan olarak, bu dosyayı mevcut dizinde arar. `--rules` argümanını kullanarak farklı bir yol belirtebilirsiniz.

Temel yapılandırma seçenekleri şunları içerir:
*   `error_patterns` & `warning_patterns`: Yaygın hata/uyarı anahtar kelimeleri için basit dize eşleşmeleri.
*   `regex_rules`: Önem seviyeleri (örn. `critical`, `high`) ile karmaşık desenleri tanımlayın.
*   `frequency_rules`: Belirli bir zaman penceresinde bir olayın kaç kez meydana gelebileceği için eşikler belirleyin.

## 🤝 Katkıda Bulunma

Katkılarınız memnuniyetle karşılanır! Rust, log analizi veya sistem izleme konusunda tutkuluysanız, depoyu çatallamaktan, sorunlar açmaktan veya çekme istekleri göndermekten çekinmeyin.

## 📜 Lisans

Bu proje Apache Lisansı 2.0 altında lisanslanmıştır. Ayrıntılar için [LICENSE](LICENSE) dosyasına bakın.