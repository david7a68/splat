---
splat_schema: 1
splat_id: 89fe3b83-8941-5c4c-8f78-9bc0f6ff966f
splat_kind: rule
splat_key: dnd2024:class_features/rogue_level_20_stroke_of_luck
splat_name: 'Level 20: Stroke of Luck'
splat_origin:
  key: dnd2024:reference/pages/063
splat_sources:
- title: SRD 5.2.1
  locator: pp. 63
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/rogue_level_20_stroke_of_luck
  parameters:
    trigger: actor.d20_test.failed
    optional: true
    replace_d20: 20
    recompute_result: true
    resource:
      id: rogue.stroke_of_luck
      maximum: 1
      recovery:
      - event: rest.short_completed
        amount: all
      - event: rest.long_completed
        amount: all
    cost: 1
    class_scope:
      class: rogue
      minimum_level: 20
  implementation: dnd2024:class_features_rogue_level_20_stroke_of_luck
splat_contracts:
- id: dnd2024:class_features_rogue_level_20_stroke_of_luck
  inputs:
    actor:
      type: participant
      required: true
    event:
      required: false
      type: event
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - d20_test.failed
  - rest.short_completed
  - rest.long_completed
  capabilities:
  - character_features
  - resources
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 63
  engine_gaps: &id001
  - dnd2024:class_features_rogue_level_20_stroke_of_luck
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 7cd40caf46d8f4be82724ab8fe9f17882b34d89fee0d08cceb3904dbde768008
  parent_name: Rogue
  line_span:
    first: 5345
    count: 6
splat_clauses:
- key: source_rules
  text: '# Level 20: Stroke of Luck You have a marvelous knack for succeeding when you need to. If you
    fail a D20 Test, you can turn the roll into a 20. Once you use this feature, you can’t use it again
    until you finish a Short or Long Rest.'
  sha256: 9ace8e7906ac0a68879f8a352725a52edc42351ba27aa93c9755416bbeeb6599
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 20: Stroke of Luck

<!-- SRD page 63 -->

You have a marvelous knack for succeeding when\
you need to. If you fail a D20 Test, you can turn the\
roll into a 20.\
Once you use this feature, you can’t use it again\
until you finish a Short or Long Rest.\
