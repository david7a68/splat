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
splat_rule:
  mechanic: dnd2024:rules/d20_tests_ability_modifier_3
  parameters:
    default_abilities:
      melee_weapon: strength
      unarmed_strike: strength
      ranged_weapon: dexterity
      spell_attack: spellcasting_feature_ability
    feature_override: true
    finesse:
      choice:
      - strength
      - dexterity
  implementation: dnd2024:rules_d20_tests_ability_modifier_3
splat_contracts:
- id: dnd2024:rules_d20_tests_ability_modifier_3
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - effects
  events:
  - attack.calculate
  capabilities:
  - attacks
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 7
  engine_gaps: &id001
  - dnd2024:rules_d20_tests_ability_modifier_3
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 46eae05aa9b1ff069f2b39f221eae9a87e88784916f41346b63bf52936c0b6bc
  parent_name: D20 Tests
  line_span:
    first: 232
    count: 15
splat_clauses:
- key: source_rules
  text: '# Ability Modifier The Attack Roll Abilities table shows which ability modifier to use for different
    types of attack rolls. Attack Roll Abilities Ability Attack Type Strength Melee attack with a weapon
    or an Unarmed Strike (see “Rules Glossary”) Dexterity Ranged attack with a weapon Varies Spell attack
    (the ability used is determined by the spellcaster’s spellcasting feature, as explained in “Spells”)
    Some features let you use different ability modifiers from those listed. For example, the Finesse
    property (see “Equipment”) lets you use Strength or Dexterity with a weapon that has that property.'
  sha256: 0d01b069bac6a6ecbf9c2506f1bd22299d87965b47cb03570fb97f35803599c2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ability Modifier

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
