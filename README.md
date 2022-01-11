# Pixlib

Projekt biblioteki pozwalającej na manipulowanie obrazami w formatach PPM i RCR, mający na celu naukę języka Rust oraz sposobów na stratną kompresję z wykorzystaniem analizy częstotlowiściowej.

## Wspierane formaty obrazów

Na potrzeby projektu został stworzony prosty format RCR *(raw cosine representation)*. Bazuje on na standardzie JPEG, ale upraszcza znacząco strukturę pliku, co pozwoliło na skupieniu się nad założeniami kompresji. Biblioteka wspiera również format PPM, co daje możliwość podglądu efektów.

## Przykład

```rust
use std::fs::File;
use pixlib::codec::*;
use pixlib::color::Lab8;
use pixlib::image::Image;

// Otwarcie pliku wejściowego, oraz utworzenie wyjściowego
let input_file = File::open("in.ppm");
let output_file = File::create("out.rcr");

// Dekodowanie wejściowego pliku PPM i konwersja na przestrzeń kolorów LAB
let img: Image<Lab8> = ppm::decode(input_file).into();

// Enkodowanie RCR o jakości 5 i zapis do pliku
rcr::encode(output_file, rcr::Settings::quality(5), &img);
```