# Santoka

> _arukeba kimpouge suwareba kimpouge_  
> if i walk the buttercups if i sit the buttercups

Translations of 668 of [Taneda Santōka](https://en.wikipedia.org/wiki/Sant%C5%8Dka_Taneda)'s free-verse haiku, including excellent translations by [Hiroaki Satō](<https://en.wikipedia.org/wiki/Hiroaki_Sato_(translator)>), [Scott Watson](https://www.instagram.com/swbotl), and [Cid Corman](https://en.wikipedia.org/wiki/Cid_Corman).

Available as JSON for easy parsing, or in the [Leaflet.md]() format for viewing in Markdown-friendly apps like [Obsidian](https://obsidian.md/).

You can explore the poems in the dataset at [lucaaurelia.com/santoka](https://lucaaurelia.com/santoka).

## Source

Many of these poems were originally compiled and digitized by Gábor Terebess for [Terebess Asia Online](https://terebess.hu/english/haiku/taneda.html). I've added metadata like publication URLs and converted to a structured format for easy parsing.

## Poems

See `./poems.json`. Poems are in this format:

```json
{
  "id": 68,
  "publicationId": 3,
  "englishText": "Absolutely no cloud I take off my hat",
  "japaneseText": "Mattaku kumo ga nai kasa o nugi"
}
```

The `japaneseText` field is missing for some poems, and can contain either romaji or kana/kanji.

## Publications

See `./publications.json`. Publications are in this format:

```json
{
  "id": 12,
  "name": "Santôka",
  "translatorIds": [11, 16],
  "year": 2006,
  "description": "Santôka: A Translation with Photographic Images. Photographs by Hakudô Inoue; book and cover design by Kazuya Takaoka; English text by Emiko Miyashita and Paul Watsky. (PIE Books, Tokyo, 2006). 400 pages",
  "url": "https://thehaikufoundation.org/omeka/items/show/2643",
  "lucaRanking": 5
}
```

| Field           | Description                                                                                                                                                                                                               |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`          | This is my best attempt to identify a single name for the publication, but it's occasionally a judgment call since the data includes personal web pages and other informal sources.                                       |
| `translatorIds` | This is an array since sometimes publications have multiple people working on the English text, like in the example above. Publications with one translator (most of them) use a one-element array: `translatorIds: [8]`. |
| `year`          | This is either a number, or `null` if I couldn't determine a publication year.                                                                                                                                            |
| `description`   | This is a free-form text description of the publication.                                                                                                                                                                  |
| `url`           | A somewhat authoritative URL for the publication, when I could find it. `null` if I couldn't.                                                                                                                             |
| `lucaRanking`   | This is a subjective ranking based on where I want the publication to show up on [lucaaurelia.com/santoka](https://lucaaurelia.com/santoka).                                                                              |

## Translators

See `./translators.json`. Translators are in this format:

```json
{
  "id": 10,
  "name": "Scott Watson"
}
```

## Installation

If you're using Rust, you can install this dataset from [crates.io]().

```bash
cargo add santoka
```

## Example usage

Parsing JSON is straightforward in most languages. Here's a JavaScript example:

```javascript
import fs from "fs";

const poemsJson = fs.readFileSync("./santoka/poems.json");
const poems = JSON.parse(poemsJson);
console.log(poems);
```

If you're using Rust, the `santoka` crate takes care of parsing for you:

```rust
fn main() {
    let dataset = santoka::Dataset::new();

    for poem in &dataset.poems {
        dbg!(&poem);

        let publication = dataset.publication(poem.publication_id);
        dbg!(&publication);

        let translators = dataset.translators(publication.translator_ids);
        dbg!(&translators);
    }
}
```
