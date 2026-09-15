---
splat_schema: 1
splat_id: 6cf285ec-2c46-540a-b012-b95e9af50229
splat_kind: rule
splat_key: dnd2024:feats/epic_boon_feats_boon_of_fate
splat_name: Boon of Fate
splat_origin:
  key: dnd2024:reference/pages/088
splat_sources:
- title: SRD 5.2.1
  locator: pp. 88
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:feats/epic_boon_feats_boon_of_fate
  parameters:
    feat_category: epic_boon
    prerequisites:
      minimum_character_level: 19
    ability_increase:
      count: 1
      amount: 1
      choices: all
      maximum_score: 30
    improve_fate:
      trigger_any:
      - creature.d20_test_succeeded
      - creature.d20_test_failed
      target:
        self_allowed: true
        range_feet: 60
      optional: true
      roll:
        count: 2
        sides: 4
      modify_choice:
      - add_to_d20
      - subtract_from_d20
      recompute_result: true
      resource:
        id: boon.fate
        maximum: 1
        recovery:
        - event: actor.initiative.rolled
          amount: all
        - event: rest.short_completed
          amount: all
        - event: rest.long_completed
          amount: all
      cost: 1
  implementation: dnd2024:feats_epic_boon_feats_boon_of_fate
splat_contracts:
- id: dnd2024:feats_epic_boon_feats_boon_of_fate
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
  - d20_test.succeeded
  - d20_test.failed
  - initiative.rolled
  - rest.short_completed
  - rest.long_completed
  capabilities:
  - character_features
  - resources
  - initiative
splat_conversion:
  status: specified
  category: feat
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 88
  engine_gaps: &id001
  - dnd2024:feats_epic_boon_feats_boon_of_fate
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: b5375c0c7d6e11a6b38be45bd66c2629d22fffaa382be7d784bd5c6cc5093e26
  parent_name: Epic Boon Feats
  line_span:
    first: 7586
    count: 11
splat_clauses:
- key: source_rules
  text: '# Boon of Fate Epic Boon Feat (Prerequisite: Level 19+) You gain the following benefits. Ability
    Score Increase. Increase one ability score of your choice by 1, to a maximum of 30. Improve Fate.
    When you or another creature within 60 feet of you succeeds on or fails a D20 Test, you can roll 2d4
    and apply the total rolled as a bonus or penalty to the d20 roll. Once you use this benefit, you can’t
    use it again until you roll Initiative or finish a Short or Long Rest.'
  sha256: 3db10bbd861e4ddd4e9d35338fd95a23a8b8d95301c990d1064ec3104302129b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Boon of Fate

<!-- SRD page 88 -->

Epic Boon Feat (Prerequisite: Level 19+)\
You gain the following benefits.\
Ability Score Increase. Increase one ability score\
of your choice by 1, to a maximum of 30.\
Improve Fate. When you or another creature\
within 60 feet of you succeeds on or fails a D20\
Test, you can roll 2d4 and apply the total rolled as a\
bonus or penalty to the d20 roll. Once you use this\
benefit, you can’t use it again until you roll Initiative\
or finish a Short or Long Rest.\
