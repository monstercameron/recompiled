# Minecraft: Java Edition 26.3 profile and Wikipedia audit

## Scope

The catalog target is the exact game key `Minecraft: Java Edition 26.3`. The associated Horizon row is a separate catalog project whose `target_type` is `Game`; this update profiles the game key only and adds no project facts.

## Full-replacement profile packet

`minecraft-java-26-3-profile-update.json` contains one profile with six fact rows. The first three are preserved from the current database exactly, including their sources, evidence text, date granularity, and `verified_on` dates:

- `original_platform`: `Minecraft Java Edition`
- `genre`: `Sandbox / survival`
- `regional_release_date`: `2026-09-15` (`day`)

The real `import_game_profiles` importer deletes all existing `game_facts` for each key included in its input, then inserts the facts in the supplied profile. Therefore this is a full replacement payload, not an additions-only payload.

Three sourced facts are appended:

- `developer`: `Mojang Studios`, from the edition-specific row in Microsoft/Xbox’s [Minecraft Franchise Fact Sheet](https://news.xbox.com/wp-content/uploads/sites/2/2021/04/Minecraft-Franchise-Fact-Sheet_Oct.-2021.pdf).
- `platforms`: `Windows, macOS, and Linux`, from Mojang’s [Minecraft Java & Bedrock PC product page](https://www.minecraft.net/en-us/store/minecraft-java-bedrock-edition-pc), which states that Java Edition runs on those three systems.
- `series`: `Minecraft`, from the [English Wikipedia Minecraft article](https://en.wikipedia.org/wiki/Minecraft), whose infobox names the series and whose text discusses the original Java Edition.

Publisher was deliberately omitted. The 2021 Microsoft/Xbox fact sheet assigns Xbox Game Studios as publisher for Java Edition, while the current general Minecraft article attributes publication to Mojang Studios and separately names Xbox Game Studios for Bedrock publishing rights. Those sources do not establish one unambiguous current publisher for the 26.3 key.

## Wikipedia link

The live game row currently uses `https://en.wikipedia.org/wiki/Special:Search?search=Minecraft%3A+Java+Edition+26.3`, with no stored title. The proposed link in `minecraft-java-26-3-wikipedia-link.json` is the direct English article `https://en.wikipedia.org/wiki/Minecraft`, title `Minecraft`. The article states that it is about the video game, has a Java Edition release section, and describes Java Edition as Mojang’s original edition. It is an appropriate page for the underlying game, though it does not identify the 26.3 update as a separate article.

## Validation

- Parsed both JSON manifests and checked exact key, required fields, HTTPS sources, evidence, verification dates, and day granularity for the dated fact.
- Confirmed the profile contains the exact three existing database facts plus the three proposed facts.
- Read the live SQLite catalog in read-only mode and confirmed that the existing three fact rows and current search-fallback Wikipedia URL remain untouched; no importer was run.
- Confirmed the associated Horizon project is a separate `Game` target. No SQLite, tracked, documentation, or build files were modified.
