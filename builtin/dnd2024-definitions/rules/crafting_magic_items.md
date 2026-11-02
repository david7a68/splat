---
splat_schema: 1
splat_id: 9d5ca422-452e-5972-b406-18b8d78a911d
splat_kind: rule
splat_key: dnd2024:rules/crafting_magic_items
splat_name: Crafting Magic Items
splat_origin:
  key: dnd2024:reference/pages/206
splat_sources:
- title: SRD 5.2.1
  locator: pp. 206
  url: https://www.dndbeyond.com/srd
---

# Crafting Magic Items

## Source text

<!-- SRD page 206 -->

“Equipment” contains rules on brewing Potions of Healing and scribing Spell Scrolls. To create other magic items, follow the rules below. In these rules, “you” refers to the character crafting the magic item.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/crafting_magic_items ` |
| ` implementation ` | ` dnd2024:rules_crafting_magic_items ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` other_items_rule_scope ` | ` magic_item_crafting ` |

#### Special crafting rule exceptions

<!-- splat:yaml path=splat_rule/parameters/special_crafting_rule_exceptions -->

```yaml
potion_of_healing:
  key: dnd2024:rules/brewing_potions_of_healing
spell_scroll:
  key: dnd2024:rules/scribing_spell_scrolls
```

#### Requirement rules

<!-- splat:rows path=splat_rule/parameters/requirement_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/crafting_magic_items_arcana_proficiency ` |
| ` 1 ` | ` dnd2024:rules/crafting_magic_items_tools ` |
| ` 2 ` | ` dnd2024:rules/crafting_magic_items_spells ` |
| ` 3 ` | ` dnd2024:rules/crafting_magic_items_time_and_cost ` |
