---
splat_schema: 1
splat_id: bbc594f9-2784-5f07-9e9f-19b1a193f626
splat_kind: rule
splat_key: dnd2024:rules/sorcerer_twinned_spell
splat_name: Twinned Spell
splat_origin:
  key: dnd2024:reference/pages/067
splat_sources:
- title: SRD 5.2.1
  locator: pp. 67
  url: https://www.dndbeyond.com/srd
---

# Twinned Spell

## Source text

<!-- SRD page 67 -->

Cost: 1 Sorcery Point\
When you cast a spell, such as Charm Person, that\
can be cast with a higher-level spell slot to target an\
additional creature, you can spend 1 Sorcery Point\
to increase the spell’s effective level by 1.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/sorcerer_twinned_spell ` |
| ` implementation ` | ` dnd2024:rule_sorcerer_twinned_spell ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` cost_sorcery_points ` | ` 1 ` |
| ` effective_spell_level_increase ` | ` 1 ` |

#### Spell eligibility

<!-- splat:values path=splat_rule/parameters/spell_eligibility -->

| Field | Value |
| --- | --- |
| ` higher_slot_level_can_add_creature_target ` | ` true ` |
