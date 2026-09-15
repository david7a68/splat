---
splat_schema: 1
splat_id: 8402faf1-91d2-5af0-83de-a1fa4f3f61a5
splat_kind: rule
splat_key: dnd2024:class_features/monk_level_4_slow_fall
splat_name: 'Level 4: Slow Fall'
splat_origin:
  key: dnd2024:reference/pages/051
splat_sources:
- title: SRD 5.2.1
  locator: pp. 51
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/monk_level_4_slow_fall
  parameters:
    trigger: self_falls
    action: reaction
    fall_damage_reduction:
      op: multiply
      args:
      - 5
      - stat: classes.monk.level
  implementation: dnd2024:class_features_monk_level_4_slow_fall
splat_contracts:
- id: dnd2024:class_features_monk_level_4_slow_fall
  inputs:
    actor:
      type: participant
      required: true
    trigger:
      required: false
      type: event
    targets:
      required: false
      type: participant_list
  outputs:
  - feature_state
  - damage
  events:
  - fall.started
  capabilities:
  - damage
  - reactions
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 51
  engine_gaps: &id001
  - dnd2024:class_features_monk_level_4_slow_fall
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: c1345e52ec8da20f6893eee2cea2cf54debad019b28a81af072e4763d3c5836e
  parent_name: Monk
  line_span:
    first: 4266
    count: 4
splat_clauses:
- key: source_rules
  text: '# Level 4: Slow Fall You can take a Reaction when you fall to reduce any damage you take from
    the fall by an amount equal to five times your Monk level.'
  sha256: 435e21d9a8210905bdb9640d5b50ff27a0c3d350773f5c9391bcb3f7a2233655
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 4: Slow Fall

<!-- SRD page 51 -->

You can take a Reaction when you fall to reduce any\
damage you take from the fall by an amount equal\
to five times your Monk level.\
