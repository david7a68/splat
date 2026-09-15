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

<!-- splat:table
path: splat_rule/parameters/table
key: Key
order:
- row_1
- row_2
- row_3
- row_4
- row_5
- row_6
- row_7
- row_8
- row_9
columns:
  Key:
    type: key
  Roll min:
    path: roll_min
    type: integer
  Roll max:
    path: roll_max
    type: integer
  Alignment:
    path: alignment
    type: symbol
-->

| Key | Roll min | Roll max | Alignment |
| --- | --- | --- | --- |
| ` row_1 ` | ` 1 ` | ` 15 ` | ` Lawful good ` |
| ` row_2 ` | ` 16 ` | ` 35 ` | ` Neutral good ` |
| ` row_3 ` | ` 36 ` | ` 50 ` | ` Chaotic good ` |
| ` row_4 ` | ` 51 ` | ` 63 ` | ` Lawful neutral ` |
| ` row_5 ` | ` 64 ` | ` 73 ` | ` Neutral ` |
| ` row_6 ` | ` 74 ` | ` 85 ` | ` Chaotic neutral ` |
| ` row_7 ` | ` 86 ` | ` 89 ` | ` Lawful evil ` |
| ` row_8 ` | ` 90 ` | ` 96 ` | ` Neutral evil ` |
| ` row_9 ` | ` 97 ` | ` 100 ` | ` Chaotic evil ` |
