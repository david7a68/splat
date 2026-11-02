---
splat_schema: 1
splat_id: 44dc08e2-2b14-5ce7-8f42-ead52582e3d2
splat_kind: item
splat_key: dnd2024:items/scimitar_of_speed
splat_name: Scimitar of Speed
splat_origin:
  key: dnd2024:reference/magic_item/scimitar_of_speed
splat_sources:
- title: SRD 5.2.1
  locator: pp. 243
  url: https://www.dndbeyond.com/srd
---

# Scimitar of Speed

## Source text

<!-- SRD page 243 -->

Weapon (Scimitar), Very Rare (Requires Attunement)

You gain a +2 bonus to attack rolls and damage rolls made with this magic weapon. In addition, you can make one attack with it as a Bonus Action on each of your turns.

Original layout: [[../../srd-5.2.1/pages/243|page 243]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` weapon ` |
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
| ` mechanic ` | ` dnd2024:items/scimitar_of_speed ` |
| ` implementation ` | ` dnd2024:item_scimitar_of_speed ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` attack_roll_bonus ` | ` 2 ` |
| ` damage_roll_bonus ` | ` 2 ` |
| ` scope ` | ` attacks_with_source_item ` |

#### Bonus action attack

<!-- splat:values path=splat_rule/parameters/bonus_action_attack -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` weapon ` | ` source_item ` |
| ` on_own_turn ` | ` true ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_weapons ` |
