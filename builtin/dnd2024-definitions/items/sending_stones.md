---
splat_schema: 1
splat_id: 257e062b-0245-5f12-9887-8d187aac2a7b
splat_kind: item
splat_key: dnd2024:items/sending_stones
splat_name: Sending Stones
splat_origin:
  key: dnd2024:reference/magic_item/sending_stones
splat_sources:
- title: SRD 5.2.1
  locator: pp. 243
  url: https://www.dndbeyond.com/srd
---

# Sending Stones

## Source text

<!-- SRD page 243 -->

Wondrous Item, Uncommon\
Sending Stones come in pairs, with each stone\
carved to match the other so the pairing is easily\
recognized. While you touch one stone, you can cast\
Sending from it. The target is the bearer of the other\
stone. If no creature bears the other stone, you\
know that fact as soon as you use the stone, and you\
don’t cast the spell.\
Once Sending is cast using either stone, the stones\
can’t be used again until the next dawn. If one of the\
stones in a pair is destroyed, the other one becomes\
nonmagical.\

Original layout: [[../../srd-5.2.1/pages/243|page 243]].

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
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/sending_stones ` |
| ` implementation ` | ` dnd2024:item_sending_stones ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` pair_size ` | ` 2 ` |
| ` equipped ` | ` touching ` |
| ` attunement_required ` | ` false ` |

#### Spells

<!-- splat:rows path=splat_rule/parameters/spells -->

| Row | target | usage_pool |
| --- | --- | --- |
| ` 0 ` | ` other_stone_bearer ` | ` shared_pair ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/0/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/sending ` |

##### Uses

<!-- splat:values path=splat_rule/parameters/spells/0/uses -->

| Field | Value |
| --- | --- |
| ` maximum ` | ` 1 ` |
| ` recovery_event ` | ` time.dawn ` |

#### No other bearer

<!-- splat:values path=splat_rule/parameters/no_other_bearer -->

| Field | Value |
| --- | --- |
| ` inform_user ` | ` true ` |
| ` cast_spell ` | ` false ` |
| ` expend_use ` | ` false ` |

#### One stone destroyed

<!-- splat:values path=splat_rule/parameters/one_stone_destroyed -->

| Field | Value |
| --- | --- |
| ` other_stone_magic ` | ` false ` |
