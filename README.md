# ✨ RustHound: Gerçek Zamanlı Log Analiz ve İzleme Aracı ✨

RustHound, Rust ile titizlikle hazırlanmış **güçlü, açık kaynaklı, gerçek zamanlı bir log analiz ve izleme aracıdır**. Sistem yöneticileri, geliştiriciler ve log verilerini sürekli olarak izlemesi gereken herkes için tasarlanmıştır.

Log dosyalarındaki önemli olayları, desenleri ve anormallikleri manuel olarak arama zahmetine veda edin. RustHound ile loglarınızı tanımladığınız kurallara göre otomatik olarak izler ve size değerli bilgiler sunar!

##  Sizi Güçlendiren Özellikler

*   **Gerçek Zamanlı Log İzleme:**  Belirtilen log dosyalarını gerçek zamanlı olarak izler ve yeni girişleri anında işler.
*   **Kural Tabanlı Desen Eşleştirme:**  Yapılandırılabilir kurallar (`rules.toml`) kullanarak log girişlerinde belirli metin desenlerini veya regex ifadelerini eşleştirir.
*   **Frekans Analizi:**  Tanımlanan desenlerin belirli zaman aralıklarında ne sıklıkta ortaya çıktığını izler ve eşik değerleri aşıldığında uyarı verir.
*   **Esnek Çıktı Seçenekleri:**  Analiz sonuçlarını konsola yazdırabilir veya yapılandırılabilir JSON dosyalarına kaydedebilir.
*   **Modüler ve Genişletilebilir Mimari:**  Rust'ta temiz, modüler bir tasarımla inşa edilmiştir, bu da yeni analiz modülleri, çıktı formatları veya log kaynakları eklemeyi kolaylaştırır.
*   **Dinamik Yapılandırma Yönetimi:**  Log dosyası yollarını, izleme aralıklarını ve kural setlerini kolayca yapılandırın.
*   **Çapraz Platform Uyumluluğu:**  Linux, macOS ve Windows'ta sorunsuz çalışır ve farklı işletim sistemlerinde tutarlı bir izleme deneyimi sunar.

## ️ Bileşenler

RustHound, log analizi ve izleme görevlerini yerine getirmek için çeşitli modüllerden oluşur:

*   `watcher`: Log dosyalarını izler ve yeni girişleri okur.
*   `analyzer`: Okunan log girişlerinde desen eşleştirme ve frekans analizi yapar.
*   `config`: Uygulama yapılandırmasını ve kurallarını yönetir.
*   `output`: Analiz sonuçlarını farklı formatlarda (konsol, JSON) çıktı verir.

## ⚡ Başlarken: RustHound'u Serbest Bırakın

### Ön Koşullar

Başlamadan önce şunlara sahip olduğunuzdan emin olun:

*   **Rust:** `rustup.rs` adresinden `rustup` aracılığıyla Rust'ı yükleyin ([https://rustup.rs/](https://rustup.rs/)).

### Kurulum

1.  **Depoyu Klonlayın:**

    ```bash
    git clone https://github.com/your-username/RustHound.git # Depo URL'sini güncelleyin
    cd RustHound
    ```

2.  **Kuralları Yapılandırın:**

    Projenin kök dizininde bulunan `rules.toml` dosyasını düzenleyerek izlemek istediğiniz desenleri ve frekans eşiklerini tanımlayın. Örnek:

    ```toml
    [[rules]]
    name = "ErrorPattern"
    pattern = "ERROR: (.*)"
    frequency_threshold = 5
    time_window_seconds = 60

    [[rules]]
    name = "WarningPattern"
    pattern = "WARNING: (.*)"
    ```

3.  **RustHound'u Derleyin:**

    ```bash
    cargo build --release
    ```

### Kullanım

RustHound'u çalıştırmak için:

```bash
cargo run --release -- /path/to/your/logfile.log
```

*   `/path/to/your/logfile.log`: İzlemek istediğiniz log dosyasının yolu.

**Örnek Kullanım:**

Mevcut dizindeki `sample.log` dosyasını izlemek için:

```bash
cargo run --release -- sample.log
```

Çıktıyı JSON dosyasına kaydetmek için (yapılandırmaya bağlı olarak):

```bash
# config/rules.rs veya main.rs içinde JSON çıktısı için gerekli yapılandırmayı yapın
cargo run --release -- sample.log
```

## ⚙️ Yapılandırma Seçenekleri

RustHound'un davranışı, `rules.toml` dosyası aracılığıyla özelleştirilebilir.

Temel yapılandırma seçenekleri şunları içerir:

*   `rules.toml`: İzlenecek desenleri, frekans eşiklerini ve zaman pencerelerini tanımlayan ana yapılandırma dosyası.

##  Katkıda Bulunma

Katkılarınızı memnuniyetle karşılıyoruz! Rust, log analizi ve sistem izleme konusunda tutkuluysanız, depoyu çatallamaktan, sorunlar açmaktan veya çekme istekleri göndermekten çekinmeyin. Ayrıntılı yönergeler için lütfen `CONTRIBUTING.md`'ye (yakında!) bakın.

##  Lisans

Bu proje Apache Lisansı 2.0 altında lisanslanmıştır. Ayrıntılar için [LICENSE](LICENSE) dosyasına bakın.

---

**RustHound** bağımsız bir projedir.