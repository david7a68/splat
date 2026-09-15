---
splat_schema: 1
splat_id: 49386059-2b85-5599-93b5-5dd5bbf41c46
splat_kind: rule
splat_key: dnd2024:feats/epic_boon_feats_boon_of_dimensional_travel
splat_name: Boon of Dimensional Travel
splat_origin:
  key: dnd2024:reference/pages/088
splat_sources:
- title: SRD 5.2.1
  locator: pp. 88
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:feats/epic_boon_feats_boon_of_dimensional_travel
  parameters:
    feat_category: epic_boon
    minimum_character_level: 19
    ability_score_increase:
      choice: any_ability
      amount: 1
      maximum: 30
    blink_steps:
      trigger: immediately_after_attack_or_magic_action
      optional: true
      teleport:
        maximum_feet: 30
        destination_unoccupied: true
        destination_visible: true
  implementation: dnd2024:feats_epic_boon_feats_boon_of_dimensional_travel
splat_contracts:
- id: dnd2024:feats_epic_boon_feats_boon_of_dimensional_travel
  inputs:
    actor:
      type: participant
      required: true
    destination:
      required: false
      type: location
  outputs:
  - feature_state
  events:
  - character.feat_selected
  - action.attack_completed
  - action.magic_completed
  capabilities:
  - feats
  - ability_scores
  - teleportation
splat_conversion:
  status: specified
  category: feat
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 88
  engine_gaps: &id001
  - dnd2024:feats_epic_boon_feats_boon_of_dimensional_travel
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 9ab02146bd4713de5b920a21848f918c21ea9566815a28759c03eff9182cc896
  parent_name: Epic Boon Feats
  line_span:
    first: 7578
    count: 8
splat_clauses:
- key: source_rules
  text: '# Boon of Dimensional Travel Epic Boon Feat (Prerequisite: Level 19+) You gain the following
    benefits. Ability Score Increase. Increase one ability score of your choice by 1, to a maximum of
    30. Blink Steps. Immediately after you take the Attack action or the Magic action, you can teleport
    up to 30 feet to an unoccupied space you can see.'
  sha256: 2a02a542d13337181521c5ac01550b823c00744aac12f178054b704b975f2d2d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Boon of Dimensional Travel

<!-- SRD page 88 -->

Epic Boon Feat (Prerequisite: Level 19+)\
You gain the following benefits.\
Ability Score Increase. Increase one ability score\
of your choice by 1, to a maximum of 30.\
Blink Steps. Immediately after you take the At-\
tack action or the Magic action, you can teleport up\
to 30 feet to an unoccupied space you can see.\
