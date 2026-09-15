---
splat_schema: 1
splat_id: ad50542a-870b-5aa1-ae1f-4092d8750fd7
splat_kind: rule
splat_key: dnd2024:feats/epic_boon_feats_boon_of_combat_prowess
splat_name: Boon of Combat Prowess
splat_origin:
  key: dnd2024:reference/pages/088
splat_sources:
- title: SRD 5.2.1
  locator: pp. 88
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:feats/epic_boon_feats_boon_of_combat_prowess
  parameters:
    feat_category: epic_boon
    prerequisites:
      minimum_character_level: 19
    ability_increase:
      count: 1
      amount: 1
      choices: all
      maximum_score: 30
    peerless_aim:
      trigger: actor.attack_missed
      optional: true
      replace_result: hit
      resource:
        id: boon.combat_prowess
        maximum: 1
        recovery:
        - event: actor.turn.start
          amount: all
      cost: 1
  implementation: dnd2024:feats_epic_boon_feats_boon_of_combat_prowess
splat_contracts:
- id: dnd2024:feats_epic_boon_feats_boon_of_combat_prowess
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
  - attack.missed
  - turn.start
  capabilities:
  - character_features
  - resources
splat_conversion:
  status: specified
  category: feat
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 88
  engine_gaps: &id001
  - dnd2024:feats_epic_boon_feats_boon_of_combat_prowess
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 2067eaf06e66249d1c4502b21f58919722cb376f49e4fde5243db8fee6b9a592
  parent_name: Epic Boon Feats
  line_span:
    first: 7570
    count: 8
splat_clauses:
- key: source_rules
  text: '# Boon of Combat Prowess Epic Boon Feat (Prerequisite: Level 19+) You gain the following benefits.
    Ability Score Increase. Increase one ability score of your choice by 1, to a maximum of 30. Peerless
    Aim. When you miss with an attack roll, you can hit instead. Once you use this benefit, you can’t
    use it again until the start of your next turn.'
  sha256: e66eef5002f8b073701db1a74671c4bfc44b973fd0bf0c9171d51303de35a525
  status: contract_specified
splat_requires_mechanics: *id001
---

# Boon of Combat Prowess

<!-- SRD page 88 -->

Epic Boon Feat (Prerequisite: Level 19+)\
You gain the following benefits.\
Ability Score Increase. Increase one ability score\
of your choice by 1, to a maximum of 30.\
Peerless Aim. When you miss with an attack roll,\
you can hit instead. Once you use this benefit, you\
can’t use it again until the start of your next turn.\
