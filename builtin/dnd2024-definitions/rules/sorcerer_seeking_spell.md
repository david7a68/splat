---
splat_schema: 1
splat_id: 0b70dc3a-8e2a-534b-a77a-522d1e5e7fe9
splat_kind: rule
splat_key: dnd2024:rules/sorcerer_seeking_spell
splat_name: Seeking Spell
splat_origin:
  key: dnd2024:reference/pages/067
splat_sources:
- title: SRD 5.2.1
  locator: pp. 67
  url: https://www.dndbeyond.com/srd
---

# Seeking Spell

## Source text

<!-- SRD page 67 -->

Cost: 1 Sorcery Point\
If you make an attack roll for a spell and miss, you\
can spend 1 Sorcery Point to reroll the d20, and you\
must use the new roll.\
You can use Seeking Spell even if you’ve already\
used a different Metamagic option during the cast-\
ing of the spell.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/sorcerer_seeking_spell ` |
| ` implementation ` | ` dnd2024:rule_sorcerer_seeking_spell ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` spell_attack_miss ` |
| ` cost_sorcery_points ` | ` 1 ` |
| ` reroll_d20 ` | ` 1 ` |
| ` must_use_new_roll ` | ` true ` |
| ` compatible_with_other_metamagic ` | ` true ` |
