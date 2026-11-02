---
splat_schema: 1
splat_id: 80c6e5dc-0a3c-5a4a-a6d5-52f6f3381221
splat_kind: rule
splat_key: dnd2024:rules/gaining_spells
splat_name: Gaining Spells
splat_origin:
  key: dnd2024:reference/pages/104
splat_sources:
- title: SRD 5.2.1
  locator: pp. 104
  url: https://www.dndbeyond.com/srd
---

# Gaining Spells

## Source text

<!-- SRD page 104 -->

Before you can cast a spell, you must have the spell prepared in your mind or have access to the spell from a magic item, such as a Spell Scroll. Your features specify which spells you have access to, if any; whether you always have certain spells prepared; and whether you can change the list of spells you have prepared.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/gaining_spells ` |
| ` implementation ` | ` dnd2024:rule_gaining_spells ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` cast_access_any ` | ` ["prepared_spell", "magic_item_spell_access"] ` |
| ` feature_controls ` | ` ["spell_access", "always_prepared", "prepared_list_changes"] ` |
