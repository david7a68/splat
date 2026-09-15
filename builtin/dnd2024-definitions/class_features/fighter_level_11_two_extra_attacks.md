---
splat_schema: 1
splat_id: 5bd10c66-1a37-5c6b-bccc-26babddfc8fd
splat_kind: rule
splat_key: dnd2024:class_features/fighter_level_11_two_extra_attacks
splat_name: 'Level 11: Two Extra Attacks'
splat_origin:
  key: dnd2024:reference/pages/048
splat_sources:
- title: SRD 5.2.1
  locator: pp. 48
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/fighter_level_11_two_extra_attacks
  parameters:
    action: attack
    turn: own
    replace_attack_count: 3
  implementation: dnd2024:class_features_fighter_level_11_two_extra_attacks
splat_contracts:
- id: dnd2024:class_features_fighter_level_11_two_extra_attacks
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
  - 48
  engine_gaps: &id001
  - dnd2024:class_features_fighter_level_11_two_extra_attacks
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: d79b8ed7b60a2e522eea8f992928d7a57805df5dca4b676502a6e993cef77364
  parent_name: Fighter
  line_span:
    first: 4032
    count: 3
splat_clauses:
- key: source_rules
  text: '# Level 11: Two Extra Attacks You can attack three times instead of once whenever you take the
    Attack action on your turn.'
  sha256: 72473a2b5bf084f1f0570392d6ad8bc4c2db033ddbb80b03e1548519088e686f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 11: Two Extra Attacks

<!-- SRD page 48 -->

You can attack three times instead of once when-\
ever you take the Attack action on your turn.\
