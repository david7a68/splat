---
splat_schema: 1
splat_id: 05caec82-0065-5872-8cc1-05fc07d9addd
splat_kind: item
splat_key: dnd2024:items/spellguard_shield
splat_name: Spellguard Shield
splat_origin:
  key: dnd2024:reference/magic_item/spellguard_shield
splat_sources:
- title: SRD 5.2.1
  locator: pp. 244
  url: https://www.dndbeyond.com/srd
---

# Spellguard Shield

## Source text

<!-- SRD page 244 -->

Armor (Shield), Very Rare (Requires Attunement)

While holding this Shield, you have Advantage on saving throws against spells and other magical effects, and spell attack rolls have Disadvantage against you.

Original layout: [[../../srd-5.2.1/pages/244|page 244]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` armor ` |
| ` rarity ` | ` very_rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/spellguard_shield ` |
| ` implementation ` | ` dnd2024:item_spellguard_shield ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` held ` |
| ` save_advantage_against ` | ` ["spells", "other_magical_effects"] ` |

#### Attacks against user disadvantage

<!-- splat:values path=splat_rule/parameters/attacks_against_user_disadvantage -->

| Field | Value |
| --- | --- |
| ` attack_category ` | ` spell ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_armor ` |
| ` 1 ` | ` dnd2024:rules/activating_a_magic_item_spells_cast_from_items ` |
