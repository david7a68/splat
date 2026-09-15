---
splat_schema: 1
splat_id: ca92ed48-3abe-527e-b1b8-40a16e208e09
splat_kind: rule
splat_key: dnd2024:class_features/hunter_level_15_superior_hunter_s_defense
splat_name: 'Level 15: Superior Hunter’s Defense'
splat_origin:
  key: dnd2024:reference/pages/061
splat_sources:
- title: SRD 5.2.1
  locator: pp. 61
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/hunter_level_15_superior_hunter_s_defense
  parameters:
    trigger: damage.taken
    action: reaction
    resistance:
      type: triggering_damage_type
      applies_to_triggering_damage: true
      until: current_turn_end
  implementation: dnd2024:class_features_hunter_level_15_superior_hunter_s_defense
splat_contracts:
- id: dnd2024:class_features_hunter_level_15_superior_hunter_s_defense
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
  - damage.taken
  capabilities:
  - damage
  - reactions
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 61
  engine_gaps: &id001
  - dnd2024:class_features_hunter_level_15_superior_hunter_s_defense
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 39555ffa82f83c8a6ed8fd6c87f2742ba39168df148afb04ebf2a66b436e9955
  parent_name: Hunter
  line_span:
    first: 5122
    count: 5
splat_clauses:
- key: source_rules
  text: '# Level 15: Superior Hunter’s Defense When you take damage, you can take a Reaction to give yourself
    Resistance to that damage and any other damage of the same type until the end of the current turn.'
  sha256: ad7bd681f05655f726379ca459d2ce96564384e0d548c75d6c3cc79f1d9caee4
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 15: Superior Hunter’s Defense

<!-- SRD page 61 -->

When you take damage, you can take a Reaction to\
give yourself Resistance to that damage and any\
other damage of the same type until the end of the\
current turn.\
