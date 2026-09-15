---
splat_schema: 1
splat_id: 4437228f-449d-529b-9a14-a144e4cc5cd3
splat_kind: rule
splat_key: dnd2024:rules/d20_tests_ability_modifier_2
splat_name: Ability Modifier
splat_origin:
  key: dnd2024:reference/pages/007
splat_sources:
- title: SRD 5.2.1
  locator: pp. 7
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/d20_tests_ability_modifier_2
  parameters:
    saving_throw_modifier: named_ability_modifier
    example_abilities:
      direct_force: strength
      dodge: dexterity
      toxic_hazard: constitution
      illusion: intelligence
      mental_assault: wisdom
      identity: charisma
  implementation: dnd2024:rules_d20_tests_ability_modifier_2
splat_contracts:
- id: dnd2024:rules_d20_tests_ability_modifier_2
  inputs:
    definition:
      type: definition
      required: true
  outputs:
  - typed_attributes
  events: []
  capabilities:
  - definition_catalogs
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 7
  engine_gaps: &id001
  - dnd2024:rules_d20_tests_ability_modifier_2
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 86c48593a5cc3a2b848d083215d8e0a5561935c0caad35ef0da685d2ed3a9eac
  parent_name: D20 Tests
  line_span:
    first: 199
    count: 14
splat_clauses:
- key: source_rules
  text: '# Ability Modifier Saving throws are named for the ability modifiers they use: a Constitution
    saving throw, a Wisdom saving throw, and so on. Different saving throws are used to resist different
    kinds of effects, as shown on the Saving Throw Examples table. Saving Throw Examples Ability Make
    a Save To … Strength Physically resist direct force Dexterity Dodge out of harm’s way Constitution
    Endure a toxic hazard Intelligence Recognize an illusion as fake Wisdom Resist a mental assault Charisma
    Assert your identity'
  sha256: 3923550a6e0ce9a2786dc155747dc084e5c384cc272b652f17d12fa32f341a5c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ability Modifier

<!-- SRD page 7 -->

Saving throws are named for the ability modifiers\
they use: a Constitution saving throw, a Wisdom\
saving throw, and so on. Different saving throws are\
used to resist different kinds of effects, as shown on\
the Saving Throw Examples table.\
Saving Throw Examples\
Ability Make a Save To …\
Strength Physically resist direct force\
Dexterity Dodge out of harm’s way\
Constitution Endure a toxic hazard\
Intelligence Recognize an illusion as fake\
Wisdom Resist a mental assault\
Charisma Assert your identity\
