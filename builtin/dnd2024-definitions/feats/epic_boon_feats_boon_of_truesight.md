---
splat_schema: 1
splat_id: bb6ef41c-4fe2-5f9a-9533-0170031a71ca
splat_kind: rule
splat_key: dnd2024:feats/epic_boon_feats_boon_of_truesight
splat_name: Boon of Truesight
splat_origin:
  key: dnd2024:reference/pages/088
splat_sources:
- title: SRD 5.2.1
  locator: pp. 88
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:feats/epic_boon_feats_boon_of_truesight
  parameters:
    feat_category: epic_boon
    prerequisites:
      minimum_character_level: 19
    ability_increase:
      count: 1
      amount: 1
      choices: all
      maximum_score: 30
    truesight_feet: 60
  implementation: dnd2024:feats_epic_boon_feats_boon_of_truesight
splat_contracts:
- id: dnd2024:feats_epic_boon_feats_boon_of_truesight
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
  - sense.query
  capabilities:
  - character_features
splat_conversion:
  status: specified
  category: feat
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 88
  engine_gaps: &id001
  - dnd2024:feats_epic_boon_feats_boon_of_truesight
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: ec643da72215781af91926658cdaccdcc1a09f9b7dadeadd35df36dc846ae81b
  parent_name: Epic Boon Feats
  line_span:
    first: 7633
    count: 7
splat_clauses:
- key: source_rules
  text: '# Boon of Truesight Epic Boon Feat (Prerequisite: Level 19+) You gain the following benefits.
    Ability Score Increase. Increase one ability score of your choice by 1, to a maximum of 30. Truesight.
    You have Truesight with a range of 60 feet.'
  sha256: 7c6a4f845dbc5f91ef0a3d76d5a638d6b0408865bcb7739676eb7b537e7fde32
  status: contract_specified
splat_requires_mechanics: *id001
---

# Boon of Truesight

<!-- SRD page 88 -->

Epic Boon Feat (Prerequisite: Level 19+)\
You gain the following benefits.\
Ability Score Increase. Increase one ability score\
of your choice by 1, to a maximum of 30.\
Truesight. You have Truesight with a range of 60\
feet.\
