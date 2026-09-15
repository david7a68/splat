---
splat_schema: 1
splat_id: 31ef89c0-c86f-5759-903b-47d89c3ff98f
splat_kind: rule
splat_key: dnd2024:class_features/barbarian_level_17_improved_brutal_strike
splat_name: 'Level 17: Improved Brutal Strike'
splat_origin:
  key: dnd2024:reference/pages/030
splat_sources:
- title: SRD 5.2.1
  locator: pp. 30
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/barbarian_level_17_improved_brutal_strike
  parameters:
    modifies:
      key: dnd2024:class_features/barbarian_level_9_brutal_strike
    extra_damage_dice:
      count: 2
      sides: 10
    effects_chosen: 2
    effects_distinct: true
    class_scope:
      class: barbarian
      minimum_level: 17
  implementation: dnd2024:class_features_barbarian_level_17_improved_brutal_strike
splat_contracts:
- id: dnd2024:class_features_barbarian_level_17_improved_brutal_strike
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
  - attack.hit
  capabilities:
  - character_features
  - damage
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 30
  engine_gaps: &id001
  - dnd2024:class_features_barbarian_level_17_improved_brutal_strike
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 03be37b029f9585516dd3ec46144fa4933dc18d767bbc152920a09ddd11f7eb7
  parent_name: Barbarian
  line_span:
    first: 2435
    count: 5
splat_clauses:
- key: source_rules
  text: '# Level 17: Improved Brutal Strike The extra damage of your Brutal Strike increases to 2d10.
    In addition, you can use two different Brutal Strike effects whenever you use your Brutal Strike feature.'
  sha256: 759a3bd6b30efa5cb61c0b8cce093c6cd236ac8d3ca27f037dd877f0f9e70430
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 17: Improved Brutal Strike

<!-- SRD page 30 -->

The extra damage of your Brutal Strike increases to\
2d10. In addition, you can use two different Brutal\
Strike effects whenever you use your Brutal Strike\
feature.\
