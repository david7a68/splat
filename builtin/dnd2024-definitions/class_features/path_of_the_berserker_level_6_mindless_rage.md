---
splat_schema: 1
splat_id: 11c332b0-68ac-58d7-a946-daa7134cee08
splat_kind: rule
splat_key: dnd2024:class_features/path_of_the_berserker_level_6_mindless_rage
splat_name: 'Level 6: Mindless Rage'
splat_origin:
  key: dnd2024:reference/pages/030
splat_sources:
- title: SRD 5.2.1
  locator: pp. 30
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/path_of_the_berserker_level_6_mindless_rage
  parameters:
    while:
      rage_active: true
    condition_immunities:
    - charmed
    - frightened
    rage_activation_ends_conditions:
    - charmed
    - frightened
    class_scope:
      class: barbarian
      minimum_level: 6
  implementation: dnd2024:class_features_path_of_the_berserker_level_6_mindless_rage
splat_contracts:
- id: dnd2024:class_features_path_of_the_berserker_level_6_mindless_rage
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
  - rage.started
  - condition.applied
  - rage.ended
  capabilities:
  - character_features
  - conditions
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 30
  engine_gaps: &id001
  - dnd2024:class_features_path_of_the_berserker_level_6_mindless_rage
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: c7c91e235560968bba44b4f5dcb9bb03824065f010a357f6882796719a147317
  parent_name: Path of the Berserker
  line_span:
    first: 2467
    count: 5
splat_clauses:
- key: source_rules
  text: '# Level 6: Mindless Rage You have Immunity to the Charmed and Frightened conditions while your
    Rage is active. If you’re Charmed or Frightened when you enter your Rage, the condition ends on you.'
  sha256: 3b86e9b25dd703104ab66e761164c182a234dd50e556921cbd47eb34caf30bb4
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 6: Mindless Rage

<!-- SRD page 30 -->

You have Immunity to the Charmed and Fright-\
ened conditions while your Rage is active. If you’re\
Charmed or Frightened when you enter your Rage,\
the condition ends on you.\
