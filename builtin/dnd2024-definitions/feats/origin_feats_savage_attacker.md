---
splat_schema: 1
splat_id: c5f74151-5da5-58ca-b7b9-b79d45c81d35
splat_kind: rule
splat_key: dnd2024:feats/origin_feats_savage_attacker
splat_name: Savage Attacker
splat_origin:
  key: dnd2024:reference/pages/087
splat_sources:
- title: SRD 5.2.1
  locator: pp. 87
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:feats/origin_feats_savage_attacker
  parameters:
    feat_category: origin
    trigger: actor.weapon_hits_target
    limit:
      count: 1
      period: turn
    optional: true
    damage_dice:
      scope: weapon_damage_dice
      roll_count: 2
      choose_one_roll_total: true
  implementation: dnd2024:feats_origin_feats_savage_attacker
splat_contracts:
- id: dnd2024:feats_origin_feats_savage_attacker
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
  - weapon.damage_roll_pending
  capabilities:
  - character_features
  - damage
splat_conversion:
  status: specified
  category: feat
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 87
  engine_gaps: &id001
  - dnd2024:feats_origin_feats_savage_attacker
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 88b069dca8c5d2825c22ad479d85707113436624bc24e01700cc86567d0c398c
  parent_name: Origin Feats
  line_span:
    first: 7508
    count: 6
splat_clauses:
- key: source_rules
  text: '# Savage Attacker Origin Feat You’ve trained to deal particularly damaging strikes. Once per
    turn when you hit a target with a weapon, you can roll the weapon’s damage dice twice and use either
    roll against the target.'
  sha256: 6e15fff8d811450532ff06bed1707f99d6e0ebbc8b9befbd581021a95a66fd43
  status: contract_specified
splat_requires_mechanics: *id001
---

# Savage Attacker

<!-- SRD page 87 -->

Origin Feat\
You’ve trained to deal particularly damaging\
strikes. Once per turn when you hit a target with\
a weapon, you can roll the weapon’s damage dice\
twice and use either roll against the target.\
