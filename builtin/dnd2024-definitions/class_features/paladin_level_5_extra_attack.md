---
splat_schema: 1
splat_id: fc81e5e9-d428-5f92-bae1-429ab6488062
splat_kind: rule
splat_key: dnd2024:class_features/paladin_level_5_extra_attack
splat_name: 'Level 5: Extra Attack'
splat_origin:
  key: dnd2024:reference/pages/055
splat_sources:
- title: SRD 5.2.1
  locator: pp. 55
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/paladin_level_5_extra_attack
  parameters:
    action: attack
    turn: own
    replace_attack_count: 2
  implementation: dnd2024:class_features_paladin_level_5_extra_attack
splat_contracts:
- id: dnd2024:class_features_paladin_level_5_extra_attack
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
  events:
  - action.attack
  capabilities:
  - attacks
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 55
  engine_gaps: &id001
  - dnd2024:class_features_paladin_level_5_extra_attack
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: f5287e7781721ca7bf8cc7bc3924f86aecd33947801dca9c27dbf354aabc2fea
  parent_name: Paladin
  line_span:
    first: 4606
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

<!-- SRD page 55 -->

You can attack twice instead of once whenever you\
take the Attack action on your turn.\
