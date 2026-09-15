---
splat_schema: 1
splat_id: 2890aabd-bede-5824-a607-9e09cb81461c
splat_kind: item
splat_key: dnd2024:items/pearl_of_power
splat_name: Pearl of Power
splat_origin:
  key: dnd2024:reference/magic_item/pearl_of_power
splat_sources:
- title: SRD 5.2.1
  locator: pp. 234
  url: https://www.dndbeyond.com/srd
---

# Pearl of Power

## Source text

<!-- SRD page 234 -->

Wondrous Item, Uncommon (Requires Attunement by\
a Spellcaster)\
While this pearl is on your person, you can take a\
Magic action to regain one expended spell slot of\
level 3 or lower. Once you use the pearl, it can’t be\
used again until the next dawn.\

Original layout: [[../../srd-5.2.1/pages/234|page 234]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` uncommon ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |
| ` attunement_requirement_text ` | ` a Spellcaster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/pearl_of_power ` |
| ` implementation ` | ` dnd2024:item_pearl_of_power ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` on_person ` |
| ` attunement_requires ` | ` spellcaster ` |
| ` action ` | ` magic ` |
| ` attunement_required ` | ` true ` |

#### Restore slot

<!-- splat:values path=splat_rule/parameters/restore_slot -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` expended_only ` | ` true ` |
| ` level_min ` | ` 1 ` |
| ` level_max ` | ` 3 ` |
| ` choice ` | ` user ` |

#### Uses

<!-- splat:values path=splat_rule/parameters/uses -->

| Field | Value |
| --- | --- |
| ` maximum ` | ` 1 ` |
| ` recovery_event ` | ` time.dawn ` |
