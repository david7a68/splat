---
splat_schema: 1
splat_id: 01354f26-fae4-5485-9c32-4dec75ed9e20
splat_kind: rule
splat_key: dnd2024:class_features/barbarian_level_1_unarmored_defense
splat_name: 'Level 1: Unarmored Defense'
splat_origin:
  key: dnd2024:reference/pages/029
splat_sources:
- title: SRD 5.2.1
  locator: pp. 29
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/barbarian_level_1_unarmored_defense
  parameters:
    when:
      armor_worn: false
    base_ac:
      op: add
      args:
      - 10
      - stat: abilities.dexterity.modifier
      - stat: abilities.constitution.modifier
    shield_permitted: true
  implementation: dnd2024:class_features_barbarian_level_1_unarmored_defense
splat_contracts:
- id: dnd2024:class_features_barbarian_level_1_unarmored_defense
  inputs:
    actor:
      type: participant
      required: true
  outputs:
  - feature_state
  events:
  - character.feature_gained
  capabilities:
  - character_features
  - effects
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 29
  engine_gaps: &id001
  - dnd2024:class_features_barbarian_level_1_unarmored_defense
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: c030956204f00ff07bd06aef2060449698bea1a54ab92836c514d07c26410e00
  parent_name: Barbarian
  line_span:
    first: 2320
    count: 5
splat_clauses:
- key: source_rules
  text: '# Level 1: Unarmored Defense While you aren’t wearing any armor, your base Armor Class equals
    10 plus your Dexterity and Constitution modifiers. You can use a Shield and still gain this benefit.'
  sha256: f5e76f6c1a20052a3737feb43cb73d025042d7e3bf805c33eb02b53c786a4a4f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 1: Unarmored Defense

<!-- SRD page 29 -->

While you aren’t wearing any armor, your base Ar-\
mor Class equals 10 plus your Dexterity and Consti-\
tution modifiers. You can use a Shield and still gain\
this benefit.\
