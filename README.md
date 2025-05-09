Nama: Faiz Assabil Firdaus  
NPM: 2306224354

# Reflection
1. What are the key differences between unary, server streaming, and bi-directional streaming RPC (Remote Procedure Call) methods, and in what scenarios would each be most suitable?

* Unary RPC adalah komunikasi satu permintaan, satu respons. Contohnya seperti login, atau cek saldo sekali klik.
* Server Streaming RPC: Server mengirim banyak respon atas satu permintaan client. Cocok untuk pengambilan data besar atau update real-time, contohnya riwayat transaksi.
* Bi-directional Streaming RPC: Client dan server bisa saling kirim dan menerima pesan berkali-kali dalam satu koneksi, cocok untuk aplikasi chat, live data feed, dsb.

2. What are the potential security considerations involved in implementing a gRPC service in Rust, particularly regarding authentication, authorization, and data encryption?

* Authentication: Pastikan request sudah terotorisasi, bisa dengan token (JWT), API key, dsb.
* Authorization: Kontrol akses agar user hanya bisa akses resource yang berhak mereka akses.
* Encryption: Gunakan TLS/SSL pada komunikasi gRPC (karena gRPC pakai HTTP/2), jaga kerahasiaan dan integritas data.
* Pastikan input tervalidasi untuk mencegah serangan injection atau serangan lain.

3. What are the potential challenges or issues that may arise when handling bidirectional streaming in Rust gRPC, especially in scenarios like chat applications?

* Concurrency: Manajemen asynchronous sehingga pesan tidak tabrakan/saling rebut.
* Ordering: Menjamin urutan pesan jika diperlukan.
* Error handling: Pastikan error di stream tidak membuat seluruh koneksi gagal.
* Backpressure: Jika salah satu pihak lambat membaca, harus bisa ditangani agar channel tidak penuh.

4. What are the advantages and disadvantages of using the tokio_stream::wrappers::ReceiverStream for streaming responses in Rust gRPC services?

* Kelebihan: Memudahkan mengkonversi mpsc channel ke stream yang kompatibel dengan tonic, code jadi lebih simpel.
* Kekurangan: Buffer terbatas, risk deadlock, dan jika channel penuh bisa menyebabkan kehilangan pesan jika tidak hati-hati.

5. In what ways could the Rust gRPC code be structured to facilitate code reuse and modularity, promoting maintainability and extensibility over time?

* Memisahkan logic bisnis ke file atau module terpisah, service implementation hanya jadi "jembatan".
* Gunakan trait untuk abstraksi behavior/service.
* Deklarasikan proto dan build system terpusat.
* Error handling terpusat dan testable.
* Hindari logic langsung di main function.

6. In the MyPaymentService implementation, what additional steps might be necessary to handle more complex payment processing logic?

* Validasi input (cek format, amount, dst).
* Integrasi ke payment gateway atau database.
* Logging dan monitoring.
* Menangani transaksi yang gagal dengan baik.
* Implementasi retry/resilience.
* Return response yang lebih detail (kode error, dsb).

7. What impact does the adoption of gRPC as a communication protocol have on the overall architecture and design of distributed systems, particularly in terms of interoperability with other technologies and platforms?

* gRPC sangat bagus untuk inter-service communication karena cross-language, performa tinggi, dan tipe data terdefinisi jelas.
* Mudah integrasi lintas bahasa (Java, Go, Python, dsb).
* Tapi untuk integrasi ke sistem legacy atau frontend (browser), perlu tambahan gateway/adapter (misal REST Gateway).

8. What are the advantages and disadvantages of using HTTP/2, the underlying protocol for gRPC, compared to HTTP/1.1 or HTTP/1.1 with WebSocket for REST APIs?

* HTTP/2 mendukung multiplexing, header compression, binary protocol (lebih efisien).
* Latency lebih rendah, cocok untuk stream/bidirectional.
* Kekurangan: Tidak semua infrastruktur/language/layer support HTTP/2/gRPC langsung, sedangkan HTTP/1.1 dan REST lebih universal.

9. How does the request-response model of REST APIs contrast with the bidirectional streaming capabilities of gRPC in terms of real-time communication and responsiveness?

* REST hanya model request-response (stateless), real-time sulit/ribet harus polling atau websocket.
* gRPC (streaming) memungkinkan real-time two-way communication secara built-in, cocok untuk app chat, notification push, dsb.

10. What are the implications of the schema-based approach of gRPC, using Protocol Buffers, compared to the more flexible, schema-less nature of JSON in REST API payloads?

* Protobuf terstruktur, validasi otomatis, lebih safe/tidak salah tulis field.
* Perlu recompilation kalau ada perubahan skema.
* JSON fleksibel, mudah di-debug, cocok untuk data yang berubah-ubah cepat, tetapi rawan typo dan tidak optimal untuk data besar.
