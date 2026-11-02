---
splat_schema: 1
splat_id: 17f98164-e101-5653-98ef-d0b0deafa222
splat_kind: rule
splat_key: dnd2024:rules/sorcerer_extended_spell
splat_name: Extended Spell
splat_origin:
  key: dnd2024:reference/pages/067
splat_sources:
- title: SRD 5.2.1
  locator: pp. 67
  url: https://www.dndbeyond.com/srd
---

# Extended Spell

## Source text

<!-- SRD page 67 -->

Cost: 1 Sorcery Point

When you cast a spell that has a duration of 1 minute or longer, you can spend 1 Sorcery Point to double its duration to a maximum duration of 24 hours.

If the affected spell requires Concentration, you have Advantage on any saving throw you make to maintain that Concentration.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/sorcerer_extended_spell ` |
| ` implementation ` | ` dnd2024:rule_sorcerer_extended_spell ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` cost_sorcery_points ` | ` 1 ` |
| ` minimum_spell_duration_minutes ` | ` 1 ` |
| ` duration_multiplier ` | ` 2 ` |
| ` duration_maximum_hours ` | ` 24 ` |
| ` concentration_saves_for_affected_spell ` | ` advantage ` |
