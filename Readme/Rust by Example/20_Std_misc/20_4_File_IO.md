## 📑 Dosya G/Ç’si (File I/O)

`File` yapısı, açılmış bir dosyayı temsil eder (bir dosya tanıtıcısını (file descriptor) sarmalar) ve altta yatan dosyaya okuma ve/veya yazma erişimi sağlar.

Dosya G/Ç’si sırasında birçok şey ters gidebileceğinden, tüm `File` metodları `io::Result<T>` türünü döndürür; bu, `Result<T, io::Error>` için bir takma addır (alias).

Bu durum, tüm G/Ç işlemlerinin başarısız olma ihtimalini açıkça gösterir. Bunun sayesinde, programcı tüm hata yollarını görebilir ve bunları proaktif bir şekilde ele almaya teşvik edilir.
