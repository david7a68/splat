---
splat_schema: 1
splat_id: 083c3735-c3bb-58b1-af7b-d6c1f0018f31
splat_kind: rule
splat_key: dnd2024:rules/sorcerer_transmuted_spell
splat_name: Transmuted Spell
splat_origin:
  key: dnd2024:reference/pages/067
splat_sources:
- title: SRD 5.2.1
  locator: pp. 67
  url: https://www.dndbeyond.com/srd
---

# Transmuted Spell

## Source text

<!-- SRD page 67 -->

Cost: 1 Sorcery Point

When you cast a spell that deals a type of damage from the following list, you can spend 1 Sorcery Point to change that damage type to one of the other listed types: Acid, Cold, Fire, Lightning, Poison, Thunder.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/sorcerer_transmuted_spell ` |
| ` implementation ` | ` dnd2024:rules_sorcerer_transmuted_spell ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_type_choice ` | ` ["acid", "cold", "fire", "lightning", "poison", "thunder"] ` |
| ` requires_original_type_in_choice ` | ` true ` |
| ` new_type_must_differ ` | ` true ` |
| ` scope ` | ` current_spell_cast ` |

#### Cost

<!-- splat:values path=splat_rule/parameters/cost -->

| Field | Value |
| --- | --- |
| ` sorcery_points ` | ` 1 ` |
