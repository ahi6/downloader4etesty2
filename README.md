# Downloader4etesty2

Neoficiální Rust knihovna a pro stahování testových otázek z oficiálního portálu [etesty2.mdcr.cz](https://etesty2.mdcr.cz).

## Odkazy

- Stahování otázek v příkazovém řádku: https://github.com/ahi6/downloader4etesty2_cli
- Grafická desktopová aplikace pro procvičování otázek: https://github.com/ahi6/autoskola_testy

## Základní použití

```rust
use downloader4etesty2::extractor::Extractor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let extractor = Extractor::new();
    
    // Získání seznamu témat
    let topics = extractor.fetch_bulletin_topics().await?;
    
    // Stažení otázek pro konkrétní téma
    let questions = extractor.fetch_questions(&topics[0].url).await?;
    
    // Stažení mediálního souboru
    let media = extractor.fetch_media_file("/relative/path/to/image.jpg").await?;
    
    Ok(())
}
```

## Upozornění

⚠️ **DŮLEŽITÉ**: Tento projekt je určen pouze pro osobní a vzdělávací účely. Respektujte prosím podmínky používání oficiálního webu etesty2.mdcr.cz.

**Tento projekt je neoficiální a není spojen s Ministerstvem dopravy ČR.**

## Licence

Tento projekt je licencován pod EUPL v1.2 licencí - viz soubory `LICENSE` (v češtině) a `LICENSE_en` (v angličtině) pro podrobnosti.

