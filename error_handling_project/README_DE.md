# Fehlerbehandlungs-Projekt (Deutsch)

![rust](https://img.shields.io/badge/Rust-1.89.0-orange)

Dieses Projekt demonstriert Rusts Fehlerbehandlung (Kapitel 9) praktisch.

Kurz zusammengefasst:
Dieses Projekt demonstriert Rusts Fehlerbehandlung (Kapitel 9) praktisch.

Kurz zusammengefasst:

- `thiserror` wurde verwendet, um `AppError` zu definieren.
- `anyhow` vereinfacht die Fehlerbehandlung im CLI-Kontext.
- `clap` stellt ein CLI mit Subcommands zur Verfügung: `read` und `generate`.

Benutzung (Schnellstart)

```bash
cd d:/rust_teorie/error_handling_project

# Tests
cargo test

# Beispiel-Datei erzeugen
cargo run -- generate sample.txt 42

# Datei lesen
cargo run -- read sample.txt
```

Erwartete Ausgaben

1. Testausgabe (kompakt):

```
running 3 tests
test tests::test_read_first_number_not_found ... ok
test tests::test_read_first_number_parse_error ... ok
test tests::test_read_first_number_ok ... ok

test result: ok. 3 passed; 0 failed
```

2. Generate Beispiel:

```
Wrote sample file: sample.txt
```

Hinweis: Der Befehl `generate` hängt den Wert an das Ende der Datei an (append) statt sie zu überschreiben.
Wenn Sie zuerst `generate sample.txt 1` und dann `generate sample.txt 2` ausführen, enthält `sample.txt` zwei Zeilen: `1` und `2`.

3. Read Beispiel:

```
First number: 42
```

Fehlerbeispiel (falls Datei text enthält):

```
cargo run -- read test_parse.txt

Error: Parse error: invalid digit found in string
```

Lerninhalte

- `Result` & `?` in der Praxis
- `thiserror` für benutzerdefinierte Fehler
- `anyhow` für ergonomisches Error-Handling mit Kontext
- `clap` für CLI-Design

Weiterführende Ideen

- JSON-Ausgabeoption (`--format json`)
- Weitere CLI-Flags (`--quiet`, `--verbose`)
- Logging und erweiterte Error-Chaining-Beispiele

Kleines ASCII-Logo

```
	_____                 _                 _
 | ____|_   _____ _ __ | |_ ___  __ _  __| | ___ _ __
 |  _| \ \ / / _ \ '_ \| __/ _ \/ _` |/ _` |/ _ \ '__|
 | |___ \ V /  __/ | | | ||  __/ (_| | (_| |  __/ |
 |_____| \_/ \___|_| |_|\__\___|\__,_|\__,_|\___|_|

 Error handling demo — thiserror + anyhow + clap
```
