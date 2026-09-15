---
splat_schema: 1
splat_id: 6cee6c8b-6b86-52a4-8cb8-da705b531ca2
splat_kind: rule
splat_key: dnd2024:class_features/champion_level_10_heroic_warrior
splat_name: 'Level 10: Heroic Warrior'
splat_origin:
  key: dnd2024:reference/pages/049
splat_sources:
- title: SRD 5.2.1
  locator: pp. 49
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/champion_level_10_heroic_warrior
  parameters:
    when:
      combat_active: true
      heroic_inspiration: false
    optional_grant: heroic_inspiration
  implementation: dnd2024:class_features_champion_level_10_heroic_warrior
splat_contracts:
- id: dnd2024:class_features_champion_level_10_heroic_warrior
  inputs:
    actor:
      type: participant
      required: true
  outputs:
  - feature_state
  events:
  - turn.started
  capabilities:
  - inspiration
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 49
  engine_gaps: &id001
  - dnd2024:class_features_champion_level_10_heroic_warrior
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: b3c4670cf90f4635c3a717b38606c5e2a86b5eaff84cb07863cc18a2ce85cf92
  parent_name: Champion
  line_span:
    first: 4069
    count: 4
splat_clauses:
- key: source_rules
  text: '# Level 10: Heroic Warrior The thrill of battle drives you toward victory. During combat, you
    can give yourself Heroic Inspiration whenever you start your turn without it.'
  sha256: 57a8471354848107ae7c96d71f0a223cec2b6cb42bc16e8271d77d6beacad493
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 10: Heroic Warrior

<!-- SRD page 49 -->

The thrill of battle drives you toward victory.\
During combat, you can give yourself Heroic Inspi-\
ration whenever you start your turn without it.\
