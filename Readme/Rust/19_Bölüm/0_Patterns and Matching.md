## 🎭 Desenler ve Eşleme (patterns and matching)

Desenler (patterns), Rust’ta hem basit hem de karmaşık türlerin yapısına karşı eşleme yapmak için kullanılan özel bir sözdizimidir. Desenleri `match` ifadeleri ve diğer yapılarla birlikte kullanmak, bir programın kontrol akışı üzerinde daha fazla denetim sağlar. Bir desen, aşağıdakilerin bazı kombinasyonlarından oluşur:

* Literaller (literals)
* Yapıları açılmış diziler (arrays), numaralandırmalar (enums), yapılar (structs) veya demetler (tuples)
* Değişkenler (variables)
* Joker karakterler (wildcards)
* Yer tutucular (placeholders)

Bazı desen örnekleri şunlardır: `x`, `(a, 3)` ve `Some(Color::Red)`. Desenlerin geçerli olduğu bağlamlarda bu bileşenler, verinin şeklini tanımlar. Programımız daha sonra değerleri desenlere göre eşleştirerek, doğru veri şekline sahip olup olmadığını ve belirli bir kod parçasını çalıştırmaya devam edip etmeyeceğini belirler.

Bir deseni kullanmak için, onu bir değerle karşılaştırırız. Eğer desen değerle eşleşirse, değerin parçalarını kodumuzda kullanırız. 6. bölümdeki desenleri kullanan `match` ifadelerini hatırlayın; örneğin para-sınıflandırma makinesi örneği. Eğer değer, desenin şekline uyuyorsa, adlandırılmış parçaları kullanabiliriz. Eğer uymuyorsa, desene bağlı kod çalıştırılmaz.

Bu bölüm, desenlerle ilgili her şey için bir başvuru kaynağıdır. Desenlerin nerelerde geçerli olduğunu, reddedilebilir (refutable) ve reddedilemez (irrefutable) desenler arasındaki farkı ve karşılaşabileceğiniz farklı desen sözdizimlerini ele alacağız. Bölümün sonunda, desenleri birçok kavramı açık bir şekilde ifade etmek için nasıl kullanacağınızı öğrenmiş olacaksınız.
