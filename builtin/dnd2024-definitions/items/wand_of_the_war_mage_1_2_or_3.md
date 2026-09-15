---
splat_schema: 1
splat_id: f3338f4c-1dec-5b81-a8b6-adebe4a319a9
splat_kind: item
splat_key: dnd2024:items/wand_of_the_war_mage_1_2_or_3
splat_name: Wand of the War Mage, +1, +2, or +3
splat_origin:
  key: dnd2024:reference/magic_item/wand_of_the_war_mage_1_2_or_3
splat_sources:
- title: SRD 5.2.1
  locator: pp. 251
  url: https://www.dndbeyond.com/srd
---

# Wand of the War Mage, +1, +2, or +3

## Source text

<!-- SRD page 251 -->

Wand, Uncommon (+1), Rare (+2), or Very Rare (+3)\
(Requires Attunement by a Spellcaster)\
While holding this wand, you gain a bonus to spell\
attack rolls determined by the wand’s rarity. In ad-\
dition, you ignore Half Cover when making a spell\
attack roll.\

Original layout: [[../../srd-5.2.1/pages/251|page 251]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wand ` |
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
| ` mechanic ` | ` dnd2024:items/wand_of_the_war_mage_1_2_or_3 ` |
| ` implementation ` | ` dnd2024:item_wand_of_the_war_mage_1_2_or_3 ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` held ` |

#### Variants

<!-- splat:table
path: splat_rule/parameters/variants
key: Key
order:
- row_1
- row_2
- row_3
columns:
  Key:
    type: key
  Rarity:
    path: rarity
    type: symbol
  Spell attack bonus:
    path: spell_attack_bonus
    type: integer
-->

| Key | Rarity | Spell attack bonus |
| --- | --- | --- |
| ` row_1 ` | ` Uncommon ` | ` 1 ` |
| ` row_2 ` | ` Rare ` | ` 2 ` |
| ` row_3 ` | ` Very rare ` | ` 3 ` |

#### Spell attacks

<!-- splat:values path=splat_rule/parameters/spell_attacks -->

| Field | Value |
| --- | --- |
| ` ignore_cover ` | ` ["half"] ` |

#### Attunement

<!-- splat:yaml path=splat_rule/parameters/attunement -->

```yaml
required: true
eligibility:
  spellcaster: true
```
