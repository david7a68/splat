---
splat_schema: 1
splat_id: 0114a2f2-f02a-5702-801b-e1aef8c5824d
splat_kind: rule
splat_key: dnd2024:class_features/evoker_level_10_empowered_evocation
splat_name: 'Level 10: Empowered Evocation'
splat_origin:
  key: dnd2024:reference/pages/082
splat_sources:
- title: SRD 5.2.1
  locator: pp. 82
  url: https://www.dndbeyond.com/srd
---

# Level 10: Empowered Evocation

## Source text

<!-- SRD page 82 -->

Whenever you cast a Wizard spell from the Evocation school, you can add your Intelligence modifier to one damage roll of that spell.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/evoker_level_10_empowered_evocation ` |
| ` implementation ` | ` dnd2024:class_features_evoker_level_10_empowered_evocation ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` spell_class ` | ` wizard ` |
| ` spell_school ` | ` evocation ` |
| ` maximum_damage_rolls_per_spell ` | ` 1 ` |

#### Optional bonus

<!-- splat:values path=splat_rule/parameters/optional_bonus -->

| Field | Value |
| --- | --- |
| ` stat ` | ` abilities.intelligence.modifier ` |
