---
splat_schema: 1
splat_id: e50cfa8d-b6fd-542b-bd8b-e14e0929fa19
splat_kind: rule
splat_key: dnd2024:class_features/monk_level_5_extra_attack
splat_name: 'Level 5: Extra Attack'
splat_origin:
  key: dnd2024:reference/pages/051
splat_sources:
- title: SRD 5.2.1
  locator: pp. 51
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/monk_level_5_extra_attack
  parameters:
    action: attack
    turn: actor
    attack_count: 2
    class_scope:
      class: monk
      minimum_level: 5
  implementation: dnd2024:class_features_monk_level_5_extra_attack
splat_contracts:
- id: dnd2024:class_features_monk_level_5_extra_attack
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
  - action.attack
  capabilities:
  - character_features
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 51
  engine_gaps: &id001
  - dnd2024:class_features_monk_level_5_extra_attack
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 4c22dc9d22dfd868a0f0e55a3745c58bc8bf95632b3ffa26c749035c19642a72
  parent_name: Monk
  line_span:
    first: 4270
    count: 3
splat_clauses:
- key: source_rules
  text: '# Level 5: Extra Attack You can attack twice instead of once whenever you take the Attack action
    on your turn.'
  sha256: 2c964ae8a86a549f83ed3a77ef83451b810103f825f5d8348ca3badddf25de65
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 5: Extra Attack

<!-- SRD page 51 -->

You can attack twice instead of once whenever you\
take the Attack action on your turn.\
