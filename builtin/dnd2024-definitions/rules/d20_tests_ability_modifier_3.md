---
splat_schema: 1
splat_id: 32c37571-8edf-547c-9f68-8cfcfe0c36d2
splat_kind: rule
splat_key: dnd2024:rules/d20_tests_ability_modifier_3
splat_name: Ability Modifier
splat_origin:
  key: dnd2024:reference/pages/007
splat_sources:
- title: SRD 5.2.1
  locator: pp. 7
  url: https://www.dndbeyond.com/srd
---

# Ability Modifier

## Source text

<!-- SRD page 7 -->

The Attack Roll Abilities table shows which ability\
modifier to use for different types of attack rolls.\
Attack Roll Abilities\
Ability Attack Type\
Strength Melee attack with a weapon or an Un-\
armed Strike (see “Rules Glossary”)\
Dexterity Ranged attack with a weapon\
Varies Spell attack (the ability used is determined\
by the spellcaster’s spellcasting feature, as\
explained in “Spells”)\
Some features let you use different ability modifiers\
from those listed. For example, the Finesse property\
(see “Equipment”) lets you use Strength or Dexter-\
ity with a weapon that has that property.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/d20_tests_ability_modifier_3 ` |
| ` implementation ` | ` dnd2024:rules_d20_tests_ability_modifier_3 ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` feature_override ` | ` true ` |

#### Default abilities

<!-- splat:values path=splat_rule/parameters/default_abilities -->

| Field | Value |
| --- | --- |
| ` melee_weapon ` | ` strength ` |
| ` unarmed_strike ` | ` strength ` |
| ` ranged_weapon ` | ` dexterity ` |
| ` spell_attack ` | ` spellcasting_feature_ability ` |

#### Finesse

<!-- splat:values path=splat_rule/parameters/finesse -->

| Field | Value |
| --- | --- |
| ` choice ` | ` ["strength", "dexterity"] ` |
