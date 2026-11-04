---
splat_schema: 2
splat_id: ebfb06ec-86bc-586d-83e5-15aa8d7d5e49
splat_kind: rule
splat_key: dnd2024:rules/crafting_magic_items_spells
splat_name: Spells
splat_origin: dnd2024:reference/pages/207
splat_sources:
- SRD 5.2.1, pp. 207, https://www.dndbeyond.com/srd
---

# Spells

## Source text

<!-- SRD page 207 -->

If a magic item allows its user to cast any spells from it, you must have all those spells prepared every day you spend crafting the item.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/crafting_magic_items_spells ` |
| ` implementation ` | ` dnd2024:rules_crafting_magic_items_spells ` |

### Parameters

#### If item grants spell casting

<!-- splat:values path=splat_rule/parameters/if_item_grants_spell_casting -->

| Field | Value |
| --- | --- |
| ` required_prepared_spells ` | ` all_spells_castable_from_item ` |
| ` timing ` | ` each_crafting_day ` |
