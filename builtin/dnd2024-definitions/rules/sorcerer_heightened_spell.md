---
splat_schema: 1
splat_id: 7d996fa1-2b9f-5ff0-8820-348733229a3c
splat_kind: rule
splat_key: dnd2024:rules/sorcerer_heightened_spell
splat_name: Heightened Spell
splat_origin:
  key: dnd2024:reference/pages/067
splat_sources:
- title: SRD 5.2.1
  locator: pp. 67
  url: https://www.dndbeyond.com/srd
---

# Heightened Spell

## Source text

<!-- SRD page 67 -->

Cost: 2 Sorcery Points

When you cast a spell that forces a creature to make a saving throw, you can spend 2 Sorcery Points to give one target of the spell Disadvantage on saves against the spell.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/sorcerer_heightened_spell ` |
| ` implementation ` | ` dnd2024:rules_sorcerer_heightened_spell ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` spell.cast_pending ` |
| ` spell_requires_saving_throw ` | ` true ` |

#### Cost

<!-- splat:values path=splat_rule/parameters/cost -->

| Field | Value |
| --- | --- |
| ` resource ` | ` sorcery_points ` |
| ` amount ` | ` 2 ` |

#### Target choice

<!-- splat:values path=splat_rule/parameters/target_choice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` from ` | ` spell_targets ` |

#### Save disadvantage

<!-- splat:values path=splat_rule/parameters/save_disadvantage -->

| Field | Value |
| --- | --- |
| ` scope ` | ` all_saves_against_this_casting ` |
