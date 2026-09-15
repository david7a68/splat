---
splat_schema: 1
splat_id: 9743460d-03fc-5d9f-8e27-0bb09f864f0c
splat_kind: rule
splat_key: dnd2024:rules/magic_item_rarity
splat_name: Magic Item Rarity
splat_origin:
  key: dnd2024:reference/pages/205
splat_sources:
- title: SRD 5.2.1
  locator: pp. 205
  url: https://www.dndbeyond.com/srd
---

# Magic Item Rarity

## Source text

<!-- SRD page 205 -->

Every magic item has a rarity, which provides a\
rough measure of an item’s power relative to other\
magic items. The rarities are shown in the Magic\
Item Rarities and Values table.\
Common magic items, such as a Potion of Healing,\
are the most plentiful. Artifacts, such as the Dragon\
Orb, are priceless, unique, and difficult to acquire.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/magic_item_rarity ` |
| ` implementation ` | ` dnd2024:rules_magic_item_rarity ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` artifact_value ` | ` priceless ` |
| ` consumable_non_scroll_multiplier ` | ` 0.5 ` |

#### Rarity values gp

<!-- splat:values path=splat_rule/parameters/rarity_values_gp -->

| Field | Value |
| --- | --- |
| ` common ` | ` 100 ` |
| ` uncommon ` | ` 400 ` |
| ` rare ` | ` 4000 ` |
| ` very_rare ` | ` 40000 ` |
| ` legendary ` | ` 200000 ` |
| ` artifact ` | ` null ` |

#### Spell scroll value

<!-- splat:values path=splat_rule/parameters/spell_scroll_value -->

| Field | Value |
| --- | --- |
| ` op ` | ` multiply ` |
| ` left ` | ` scribe_cost ` |
| ` right ` | ` 2 ` |
