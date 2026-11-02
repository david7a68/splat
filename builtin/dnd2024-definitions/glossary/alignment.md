---
splat_schema: 1
splat_id: 6cbca415-8b1a-5a63-922e-5bd896d157bb
splat_kind: rule
splat_key: dnd2024:glossary/alignment
splat_name: Alignment
splat_origin:
  key: dnd2024:reference/glossary/alignment
splat_sources:
- title: SRD 5.2.1
  locator: pp. 176
  url: https://www.dndbeyond.com/srd
---

# Alignment

## Source text

<!-- SRD page 176 -->

A creature’s alignment broadly describes its ethical attitudes and ideals. Alignment is a combination of two factors: one identifies morality (good, evil, or neutral), and the other describes attitudes toward order (lawful, chaotic, or neutral). These factors allow for nine possible combinations, such as Lawful Good and Neutral Evil. See also “Character Creation” (“Create Your Character”).

Original layout: [[../../srd-5.2.1/pages/176|page 176]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/alignment ` |
| ` implementation ` | ` dnd2024:glossary_alignment ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` morality ` | ` ["good", "evil", "neutral"] ` |
| ` order ` | ` ["lawful", "chaotic", "neutral"] ` |
| ` combination ` | ` cartesian_product ` |
| ` combination_count ` | ` 9 ` |
