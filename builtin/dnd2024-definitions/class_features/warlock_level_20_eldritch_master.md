---
splat_schema: 1
splat_id: d6b7c2ab-f7f5-5dd5-a70a-9c006cdae17a
splat_kind: rule
splat_key: dnd2024:class_features/warlock_level_20_eldritch_master
splat_name: 'Level 20: Eldritch Master'
splat_origin:
  key: dnd2024:reference/pages/072
splat_sources:
- title: SRD 5.2.1
  locator: pp. 72
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/warlock_level_20_eldritch_master
  parameters:
    feature: magical_cunning
    replace_recovery_amount: all_expended_pact_magic_slots
  implementation: dnd2024:class_features_warlock_level_20_eldritch_master
splat_contracts:
- id: dnd2024:class_features_warlock_level_20_eldritch_master
  inputs:
    actor:
      type: participant
      required: true
  outputs:
  - feature_state
  - resource_changes
  events:
  - magical_cunning.completed
  capabilities:
  - resources
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 72
  engine_gaps: &id001
  - dnd2024:class_features_warlock_level_20_eldritch_master
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: e1cf3b73cdd286863b2c3f971b0259a854fdc244d4504fd84507d4d38ece71ff
  parent_name: Warlock
  line_span:
    first: 6138
    count: 3
splat_clauses:
- key: source_rules
  text: '# Level 20: Eldritch Master When you use your Magical Cunning feature, you regain all your expended
    Pact Magic spell slots.'
  sha256: 08b3732a37440ca4c603a6502b916dfbdc67dcffb1e780c656d52ff399deba8a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 20: Eldritch Master

<!-- SRD page 72 -->

When you use your Magical Cunning feature, you\
regain all your expended Pact Magic spell slots.\
