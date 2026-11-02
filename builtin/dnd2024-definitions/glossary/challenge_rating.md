---
splat_schema: 1
splat_id: 8bb25829-19e0-542a-95b9-e6e909c45197
splat_kind: rule
splat_key: dnd2024:glossary/challenge_rating
splat_name: Challenge Rating
splat_origin:
  key: dnd2024:reference/glossary/challenge_rating
splat_sources:
- title: SRD 5.2.1
  locator: pp. 178
  url: https://www.dndbeyond.com/srd
---

# Challenge Rating

## Source text

<!-- SRD page 178 -->

Challenge Rating (CR) summarizes the threat a monster poses to a group of four player characters. Compare a monster’s CR to the characters’ level. If the CR is higher, the monster is likely a danger. If the CR is lower, the monster likely poses little threat. But circumstances and the number of player characters can significantly alter how threatening a monster is in actual play. “Gameplay Toolbox” (“Combat Encounters”) provides guidance to the GM on using CR while planning potential combat encounters. See also “Stat Block.”

Original layout: [[../../srd-5.2.1/pages/178|page 178]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/challenge_rating ` |
| ` implementation ` | ` dnd2024:glossary_challenge_rating ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` reference_party_size ` | ` 4 ` |
| ` estimate_modifiers ` | ` ["circumstances", "party_size"] ` |
| ` estimate_is_binding ` | ` false ` |

#### Comparison to character level

<!-- splat:values path=splat_rule/parameters/comparison_to_character_level -->

| Field | Value |
| --- | --- |
| ` greater ` | ` likely_danger ` |
| ` less ` | ` likely_little_threat ` |
