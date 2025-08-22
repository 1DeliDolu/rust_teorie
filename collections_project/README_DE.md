# 📚 Collections Project - Persönliches Bibliotheksverwaltungssystem

Dieses Projekt wurde erstellt, um die Themen aus Kapitel 8 ("Common Collections") von Rust praktisch zu lernen: Vektoren, Strings und HashMaps.

## 🎯 Ziel des Projekts

In diesem Projekt lernst du drei zentrale Collection-Typen in Rust kennen:

- **📋 Vector (`Vec<T>`)**: Dynamische Listen
- **🔤 String**: UTF-8-codierte Textverarbeitung
- **📦 HashMap**: Schlüssel-Wert-Speicherung

## 🚀 Funktionen

### 📖 Buchverwaltung

- ✅ Neues Buch hinzufügen
- ✅ Buchinformationen aktualisieren
- ✅ Buch entfernen
- ✅ Buchsuche (Titel und Autor)

### 📚 Bibliotheksoperationen

- ✅ Buch ausleihen / zurückgeben
- ✅ Ausleih-Historie
- ✅ Nach Autor filtern
- ✅ Statistiken anzeigen

### 🔍 Suchfunktionen

- ✅ Buch per ISBN finden
- ✅ Textbasierte Suche
- ✅ Gruppierung nach Autor

## 📁 Projektstruktur

```
collections_project/
├── src/
│   ├── main.rs          # Hauptprogramm und Benutzeroberfläche
│   ├── lib.rs           # Test-Suite
│   ├── book.rs          # Book-Struct und String-Operationen
│   └── library.rs       # Bibliotheksverwaltung - Vector & HashMap
├── Cargo.toml           # Projektkonfiguration
└── README.md            # Engl./TR Haupt-README
```

## 🏃‍♂️ Ausführen

### Programm starten

```bash
cd collections_project
cargo run
```

### Tests ausführen

```bash
cargo test
```

### Dokumentation erzeugen

```bash
cargo doc --open
```

## 📚 Zu lernende Themen

### 1. Vector (`Vec<T>`) Verwendung

- `Vec::new()` für neue Vektoren
- `push()` zum Hinzufügen von Elementen
- `get()` und Indexzugriff
- `iter()` zum Iterieren
- `filter()`, `map()`, `collect()` für funktionale Muster

### 2. String-Operationen

- `String::new()` und `String::from()`
- `push_str()` und `push()`
- `format!()` Macro zum Formatieren
- `to_lowercase()`, `contains()` und ähnliche Methoden
- UTF-8 Unterstützung

### 3. HashMap Verwendung

- `HashMap::new()` erstellen
- `insert()` zum Hinzufügen
- `get()` und `get_mut()` zum Zugriff
- `entry()` API für fortgeschrittene Muster
- `keys()`, `values()`, `iter()` zum Durchlaufen

## 💡 Code-Beispiele

### Vector

```rust
let mut books = Vec::new();
books.push(book);

let available_books: Vec<&Book> = books
	.iter()
	.filter(|book| book.available)
	.collect();
```

### HashMap

```rust
let mut books = HashMap::new();
books.insert(isbn.clone(), book);

if let Some(book) = books.get(&isbn) {
	println!("Buch gefunden: {}", book.title);
}
```

### String

```rust
let title = String::from("Rust Programming");
let info = format!("📖 {} - {} ({})", title, author, year);

if title.to_lowercase().contains(&query.to_lowercase()) {
	// Treffer
}
```

## 🧪 Testszenarien

Das Projekt enthält Unit-Tests für:

- ✅ Erstellung von Büchern und Tests der Struct-Felder
- ✅ Ausleih-/Rückgabe-Logik
- ✅ Suchfunktionalität
- ✅ Vector-Operationen
- ✅ HashMap-Operationen
- ✅ String-Manipulationen

## 📋 Schritt-für-Schritt Lernprozess

### Schritt 1: Grundstrukturen verstehen

1. `book.rs` anschauen — String-Operationen
2. `library.rs` anschauen — HashMap & Vector

### Schritt 2: Interaktiv testen

1. `cargo run` ausführen
2. Alle Menüoptionen durchprobieren
3. Fehlerfälle testen

### Schritt 3: Tests ausführen

1. `cargo test` ausführen
2. Jeden Testfall ansehen und nachvollziehen

### Schritt 4: Code erweitern

1. Neue Features hinzufügen
2. Weitere Tests schreiben
3. Performance-Verbesserungen

## 🔧 Verbesserungsideen

- 📅 Filter nach Datum
- 📊 Detailliertere Statistiken
- 💾 Speichern/Laden von Daten (Datei/JSON)
- 🏷️ Kategorien/Tags
- 📱 JSON Export/Import

## 📚 Wichtige Punkte zu Rust Collections

### Vector

- Dynamisches Array auf dem Heap
- Speichert Elemente gleichen Typs
- Index-basierter Zugriff
- Automatisches Wachstum

### String

- UTF-8 kodierte Bytes
- Auf dem Heap gelagert
- Mutable und immutable Varianten
- Umfangreiche API für Textverarbeitung

### HashMap

- Key-Value Speichermodell
- Durchschnittliche Zugriffszeit O(1)
- Keys und Values besitzen Besitz (owned)
- Unterstützt Iteration

---

## 🎯 Collections Project – Vollständige Schritt-für-Schritt-Anleitung (Deutsch)

Die folgende strukturierte Anleitung hilft beim gezielten Lernen und Anwenden der Konzepte.

### 📋 PHASE 1: Projektentdeckung & Setup (5–10 Minuten)

<details>
<summary>🔽 <strong>1.1 Projektdateien ansehen</strong></summary>

```
collections_project/
├── 🎮 src/main.rs          # Interaktives Menü
├── 🧪 src/lib.rs           # Tests
├── 📖 src/book.rs          # String-Verarbeitung
├── 📚 src/library.rs       # HashMap & Vec Logik
├── 💡 examples/            # Beispiele
└── 📚 README.md            # Haupt-README
```

**✅ To‑Dos:**

- [ ] Dateien im Editor öffnen
- [ ] Syntaxprüfung
- [ ] Struktur überfliegen

</details>

<details>
<summary>🔽 <strong>1.2 Erstes Starten</strong></summary>

```bash
cd collections_project
cargo check
cargo build
cargo run
```

Erwartete Ausgabe:

```
📚 Kişisel Kütüphane Yönetim Sistemi
=====================================
✅ 5 örnek kitap kütüphaneye eklendi!

📋 MENÜ:
1. 📖 Kitap Ekle
2. 📚 Tüm Kitapları Listele
...
```

</details>

---

### 📋 PHASE 2: Collections-Theorie praktisch lernen (20–30 Minuten)

<details>
<summary>🔽 <strong>2.1 Vector Deep Dive</strong></summary>

Fokus: `src/library.rs`

```rust
let mut borrow_history: Vec<String> = Vec::new();
self.borrow_history.push(format!("..."));

let available_books: Vec<&Book> = self.books
	.values()
	.filter(|book| book.available)
	.collect();
```

Praktische Aufgaben:

- [ ] `list_all_books()` prüfen
- [ ] `get_borrow_history()` nachverfolgen
- [ ] `find_books_by_author()` verstehen

</details>

<details>
<summary>🔽 <strong>2.2 String Mastery</strong></summary>

Fokus: `src/book.rs`

```rust
String::from(title);
title.to_string();
String::new();

format!("📖 {} - {} ({})", title, author, year);
title.to_lowercase().contains(&query.to_lowercase());
```

Aufgaben:

- [ ] `get_full_info()` testen
- [ ] `update_title()` vs `update_author()`
- [ ] UTF-8 Tests (Türkisch, Emojis)

</details>

<details>
<summary>🔽 <strong>2.3 HashMap Deep Dive</strong></summary>

Fokus: `src/library.rs` – HashMap Operationen

```rust
books.insert(isbn.clone(), book);
books.get(&isbn);
books.get_mut(&isbn);
books.remove(&isbn);

self.books_by_author
	.entry(author.clone())
	.or_insert_with(Vec::new)
	.push(isbn.clone());
```

Aufgaben:

- [ ] `add_book()` Logik prüfen
- [ ] `find_book_by_isbn()` vs `find_books_by_author()`
- [ ] `get_stats()` analysieren

</details>

---

### 📋 PHASE 3: Interaktives Testen & Unit Tests (15–20 Minuten)

<details>
<summary>🔽 <strong>3.1 Benutzer-Szenarien</strong></summary>

Starte das Programm und probiere:

```bash
cargo run
```

Testfälle:

- Neues Buch hinzufügen (Türkische Titel, lange Namen)
- Suche nach „rust“, nach Autor
- Buch ausleihen/retournieren
- Fehlerfälle (nicht vorhandene ISBN etc.)

</details>

<details>
<summary>🔽 <strong>3.2 Unit Test Deep Dive</strong></summary>

```bash
cargo test -- --nocapture
cargo test test_vector_operations
```

Tests anschauen und verstehen:

- `test_create_book`
- `test_library_search_functionality`
- `test_hashmap_operations`
- `test_vector_operations`

</details>

---

### 📋 PHASE 4: Beispiele laufen lassen (10–15 Minuten)

```bash
cargo run --example collections_examples
```

Wichtige Beispiele:

- `word_count` HashMap Beispiel
- Student grades System
- Dokumenten-Index

---

### 📋 PHASE 5: Eigene Erweiterungen (20+ Minuten)

Challenges & Verbesserungen:

- Kategorien hinzufügen (`HashMap<String, Vec<String>>`)
- JSON Import/Export (`serde`)
- Performance Messungen
- Concurrency (Arc<Mutex<>>)

Codequalität:

```bash
cargo fmt
cargo clippy
cargo doc --open
```

---

### 🎯 PHASE 6: Lernkontrolle & Nächste Schritte

Selbsttests:

- `Vec::new()` vs `vec![]`
- `String` vs `&str`
- Entry API verstehen

Nächste Themen:

1. Error Handling (Kapitel 9)
2. Generics & Lifetimes (Kapitel 10)
3. Tests (Kapitel 11)

---

### 💡 Pro Tipps

- Vector: `with_capacity()` verwenden, wenn Größe bekannt
- String: `push_str()` als performant alternative
- HashMap: `entry()` API nutzt man, um Doppel-Lookups zu vermeiden

### ⚠️ Häufige Fallen

- Borrow Checker bei gleichzeitiger Mutation während Iteration
- Unnötiges `clone()` vermeiden
- Große Collections: Heap-Fragmentierung beachten

---

Mit dieser Anleitung lernst du Theorie und Praxis der Rust-Collections kombiniert — viel Erfolg! 🦀

```

```
