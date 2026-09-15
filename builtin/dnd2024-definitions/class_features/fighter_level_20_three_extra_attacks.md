---
splat_schema: 1
splat_id: 2fd9f17f-494c-5aa3-a3fe-9e4f05945048
splat_kind: rule
splat_key: dnd2024:class_features/fighter_level_20_three_extra_attacks
splat_name: 'Level 20: Three Extra Attacks'
splat_origin:
  key: dnd2024:reference/pages/048
splat_sources:
- title: SRD 5.2.1
  locator: pp. 48
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/fighter_level_20_three_extra_attacks
  parameters:
    action: attack
    turn: actor
    attack_count: 4
    class_scope:
      class: fighter
      minimum_level: 20
  implementation: dnd2024:class_features_fighter_level_20_three_extra_attacks
splat_contracts:
- id: dnd2024:class_features_fighter_level_20_three_extra_attacks
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
  - 48
  engine_gaps: &id001
  - dnd2024:class_features_fighter_level_20_three_extra_attacks
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 47c0e7704d5425b34fd30db1787c50ce8a057e27e7ccbd05edd3a66f351c6e62
  parent_name: Fighter
  line_span:
    first: 4045
    count: 3
splat_clauses:
- key: source_rules
  text: '# Level 20: Three Extra Attacks You can attack four times instead of once whenever you take the
    Attack action on your turn.'
  sha256: 834004851b43572c76ae43083eb1a04131ba97bcc3b3b0adfad9b4481be85881
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 20: Three Extra Attacks

<!-- SRD page 48 -->

You can attack four times instead of once whenever\
you take the Attack action on your turn.\
