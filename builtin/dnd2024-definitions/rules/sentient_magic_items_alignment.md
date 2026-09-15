---
splat_schema: 1
splat_id: 24376721-4c66-52f8-9073-03ca25709563
splat_kind: rule
splat_key: dnd2024:rules/sentient_magic_items_alignment
splat_name: Alignment
splat_origin:
  key: dnd2024:reference/pages/207
splat_sources:
- title: SRD 5.2.1
  locator: pp. 207
  url: https://www.dndbeyond.com/srd
---

# Alignment

## Source text

<!-- SRD page 207 -->

A sentient magic item has an alignment. Its creator\
or nature might suggest an alignment. Otherwise,\
pick an alignment or roll on the Sentient Item’s\
Alignment table.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/sentient_magic_items_alignment ` |
| ` implementation ` | ` dnd2024:rules_sentient_magic_items_alignment ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` alignment_required ` | ` true ` |
| ` methods ` | ` ["creator_or_nature", "choose", "roll"] ` |

#### Roll

<!-- splat:values path=splat_rule/parameters/roll -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` sides ` | ` 100 ` |
| ` add ` | ` 0 ` |

#### Table

<!-- splat:rows path=splat_rule/parameters/table -->

| Row | roll_min | roll_max | alignment |
| --- | --- | --- | --- |
| ` 0 ` | ` 1 ` | ` 15 ` | ` lawful_good ` |
| ` 1 ` | ` 16 ` | ` 35 ` | ` neutral_good ` |
| ` 2 ` | ` 36 ` | ` 50 ` | ` chaotic_good ` |
| ` 3 ` | ` 51 ` | ` 63 ` | ` lawful_neutral ` |
| ` 4 ` | ` 64 ` | ` 73 ` | ` neutral ` |
| ` 5 ` | ` 74 ` | ` 85 ` | ` chaotic_neutral ` |
| ` 6 ` | ` 86 ` | ` 89 ` | ` lawful_evil ` |
| ` 7 ` | ` 90 ` | ` 96 ` | ` neutral_evil ` |
| ` 8 ` | ` 97 ` | ` 100 ` | ` chaotic_evil ` |
