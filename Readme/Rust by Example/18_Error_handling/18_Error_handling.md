## ⚠️ Hata Yönetimi (error handling)

Hata yönetimi (error handling), bir başarısızlık olasılığının ele alınması sürecidir. Örneğin, bir dosyanın okunamaması ve ardından bu hatalı girdinin kullanılmaya devam edilmesi açıkça sorunlu olurdu. Hataları fark edip açıkça yönetmek, programın geri kalanını çeşitli tehlikelerden korur.

Rust'ta hatalarla başa çıkmanın çeşitli yolları vardır ve bunlar aşağıdaki alt bölümlerde açıklanacaktır. Hepsinin az çok ince farkları ve farklı kullanım durumları vardır. Genel kural olarak:

* Açık bir `panic` ifadesi, esas olarak testler ve geri döndürülemez (unrecoverable) hatalarla başa çıkmak için faydalıdır. Prototipleme sırasında da yararlı olabilir, örneğin henüz uygulanmamış fonksiyonlarla uğraşırken; ancak bu durumlarda daha açıklayıcı olan `unimplemented` tercih edilmelidir. Testlerde `panic`, açık bir şekilde başarısız olmak için makul bir yoldur.

* `Option` türü (type), bir değerin isteğe bağlı olduğu veya değerin olmamasının bir hata durumu sayılmadığı durumlar için kullanılır. Örneğin, bir dizinin üst dizini — `/` ve `C:` için böyle bir üst dizin yoktur. `Option` ile çalışırken, `unwrap` prototipleme için veya kesinlikle bir değerin garanti olduğu durumlarda uygundur. Ancak, `expect` daha kullanışlıdır çünkü bir şeyler ters giderse özel bir hata mesajı belirtmenize imkân tanır.

* Bir şeylerin yanlış gitme olasılığı olduğunda ve çağıranın (caller) bu problemle başa çıkması gerektiğinde `Result` kullanılır. `Result` üzerinde de `unwrap` ve `expect` kullanılabilir (bunu test veya hızlı bir prototip dışında lütfen yapmayın).

Hata yönetimine dair daha ayrıntılı bir tartışma için resmi kitaptaki hata yönetimi bölümüne bakınız.
