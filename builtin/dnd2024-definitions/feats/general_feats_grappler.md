---
splat_schema: 1
splat_id: 847a6cdf-014e-50fd-a776-79ec3e16b7f4
splat_kind: rule
splat_key: dnd2024:feats/general_feats_grappler
splat_name: Grappler
splat_origin:
  key: dnd2024:reference/pages/087
splat_sources:
- title: SRD 5.2.1
  locator: pp. 87
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:feats/general_feats_grappler
  parameters:
    feat_category: general
    prerequisites:
      minimum_character_level: 4
      any_ability_score_minimum:
        abilities:
        - strength
        - dexterity
        minimum: 13
    ability_score_increase:
      choice:
      - strength
      - dexterity
      amount: 1
      maximum: 20
    punch_and_grab:
      trigger:
        unarmed_strike_hit_creature: true
        part_of_attack_action: true
        turn: own
      allow_both_options:
      - damage
      - grapple
      limit:
        count: 1
        period: turn
    attack_advantage:
      target_grappled_by: self
    fast_wrestler:
      target_grappled_by: self
      maximum_target_size: own_size
      extra_movement_cost: 0
  implementation: dnd2024:feats_general_feats_grappler
splat_contracts:
- id: dnd2024:feats_general_feats_grappler
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
    destination:
      required: false
      type: location
  outputs:
  - feature_state
  - movement
  events:
  - character.feat_selected
  - attack.hit
  - attack.pending
  - movement.pending
  capabilities:
  - feats
  - ability_scores
  - grappling
  - attacks
  - movement
splat_conversion:
  status: specified
  category: feat
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 87
  engine_gaps: &id001
  - dnd2024:feats_general_feats_grappler
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 57981c4718207ef89746595d67983e9dd22c4f4a1b384f29e4a99734b797f19c
  parent_name: General Feats
  line_span:
    first: 7528
    count: 15
splat_clauses:
- key: source_rules
  text: '# Grappler General Feat (Prerequisite: Level 4+, Strength or Dexterity 13+) You gain the following
    benefits. Ability Score Increase. Increase your Strength or Dexterity score by 1, to a maximum of
    20. Punch and Grab. When you hit a creature with an Unarmed Strike as part of the Attack action on
    your turn, you can use both the Damage and the Grapple option. You can use this benefit only once
    per turn. Attack Advantage. You have Advantage on attack rolls against a creature Grappled by you.
    Fast Wrestler. You don’t have to spend extra movement to move a creature Grappled by you if the creature
    is your size or smaller.'
  sha256: c5b803010df2b61f8f03946fc07d343e162715871002a9d53b8a05edc467b8bf
  status: contract_specified
splat_requires_mechanics: *id001
---

# Grappler

<!-- SRD page 87 -->

General Feat (Prerequisite: Level 4+, Strength or\
Dexterity 13+)\
You gain the following benefits.\
Ability Score Increase. Increase your Strength or\
Dexterity score by 1, to a maximum of 20.\
Punch and Grab. When you hit a creature with an\
Unarmed Strike as part of the Attack action on your\
turn, you can use both the Damage and the Grapple\
option. You can use this benefit only once per turn.\
Attack Advantage. You have Advantage on attack\
rolls against a creature Grappled by you.\
Fast Wrestler. You don’t have to spend extra\
movement to move a creature Grappled by you if the\
creature is your size or smaller.\
