---
splat_schema: 1
splat_id: 70523a2d-8d24-57a8-bb2b-89db5b614d56
splat_kind: rule
splat_key: dnd2024:class_features/ranger_level_17_precise_hunter
splat_name: 'Level 17: Precise Hunter'
splat_origin:
  key: dnd2024:reference/pages/059
splat_sources:
- title: SRD 5.2.1
  locator: pp. 59
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/ranger_level_17_precise_hunter
  parameters:
    advantage:
      attack_target: actor.current_hunters_mark_target
    class_scope:
      class: ranger
      minimum_level: 17
  implementation: dnd2024:class_features_ranger_level_17_precise_hunter
splat_contracts:
- id: dnd2024:class_features_ranger_level_17_precise_hunter
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
  - attack.roll
  capabilities:
  - character_features
  - d20_tests
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 59
  engine_gaps: &id001
  - dnd2024:class_features_ranger_level_17_precise_hunter
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: c47cfbbf5838990b9385eecc673eb62a041681c9fb0980822d7ebded466d638e
  parent_name: Ranger
  line_span:
    first: 4999
    count: 3
splat_clauses:
- key: source_rules
  text: '# Level 17: Precise Hunter You have Advantage on attack rolls against the creature currently
    marked by your Hunter’s Mark.'
  sha256: 9220ab21c75ef273f89b9b320b3ad7c5b84344e1202de6eafc38639a46dbe699
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 17: Precise Hunter

<!-- SRD page 59 -->

You have Advantage on attack rolls against the crea-\
ture currently marked by your Hunter’s Mark.\
