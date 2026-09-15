---
splat_schema: 1
splat_id: 3cb6ef8d-c86d-5095-8559-a10f2dacd398
splat_kind: rule
splat_key: dnd2024:class_features/monk_level_1_unarmored_defense
splat_name: 'Level 1: Unarmored Defense'
splat_origin:
  key: dnd2024:reference/pages/050
splat_sources:
- title: SRD 5.2.1
  locator: pp. 50
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/monk_level_1_unarmored_defense
  parameters:
    requires:
      armor_worn: false
      shield_wielded: false
    base_armor_class:
      op: add
      args:
      - 10
      - var: actor.abilities.dexterity.modifier
      - var: actor.abilities.wisdom.modifier
    class_scope:
      class: monk
      minimum_level: 1
  implementation: dnd2024:class_features_monk_level_1_unarmored_defense
splat_contracts:
- id: dnd2024:class_features_monk_level_1_unarmored_defense
  inputs:
    actor:
      type: participant
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - armor_class.calculate
  capabilities:
  - character_features
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 50
  engine_gaps: &id001
  - dnd2024:class_features_monk_level_1_unarmored_defense
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: e0d31bb6786a0027fa39340bac25b18126dd1f07bde25f0f99b2d3ae841111c9
  parent_name: Monk
  line_span:
    first: 4194
    count: 4
splat_clauses:
- key: source_rules
  text: '# Level 1: Unarmored Defense While you aren’t wearing armor or wielding a Shield, your base Armor
    Class equals 10 plus your Dexterity and Wisdom modifiers.'
  sha256: 5104f3a3260cccca5d4479b60e5cd9969415b7892a5bcdf1bc7f5d6b34048468
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 1: Unarmored Defense

<!-- SRD page 50 -->

While you aren’t wearing armor or wielding a\
Shield, your base Armor Class equals 10 plus your\
Dexterity and Wisdom modifiers.\
