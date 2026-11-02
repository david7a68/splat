---
splat_schema: 1
splat_id: d6a17aa0-2350-5d92-a070-6a74eb76bf95
splat_kind: item
splat_key: dnd2024:items/cloak_of_elvenkind
splat_name: Cloak of Elvenkind
splat_origin:
  key: dnd2024:reference/magic_item/cloak_of_elvenkind
splat_sources:
- title: SRD 5.2.1
  locator: pp. 215
  url: https://www.dndbeyond.com/srd
---

# Cloak of Elvenkind

## Source text

<!-- SRD page 215 -->

Wondrous Item, Uncommon (Requires Attunement)

While you wear this cloak, Wisdom (Perception) checks made to perceive you have Disadvantage, and you have Advantage on Dexterity (Stealth) checks.

Original layout: [[../../srd-5.2.1/pages/215|page 215]].

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

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/cloak_of_elvenkind ` |
| ` implementation ` | ` dnd2024:item_cloak_of_elvenkind ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` worn ` |
| ` attunement_required ` | ` true ` |

#### Checks

<!-- splat:table
path: splat_rule/parameters/checks
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Ability:
    path: ability
    type: symbol
  Skill:
    path: skill
    type: symbol
  Actor:
    path: actor
    type: symbol
  Target:
    path: target
    type: symbol
    optional: true
  Mode:
    path: mode
    type: symbol
-->

| Key | Ability | Skill | Actor | Target | Mode |
| --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` Wisdom ` | ` Perception ` | ` Other ` | ` Wearer ` | ` Disadvantage ` |
| ` row_2 ` | ` Dexterity ` | ` Stealth ` | ` Wearer ` |  | ` Advantage ` |
