---
splat_schema: 1
splat_id: 1fa8b1b5-a6b8-58bc-976d-d30efb2a87f0
splat_kind: rule
splat_key: dnd2024:feats/epic_boon_feats_boon_of_the_night_spirit
splat_name: Boon of the Night Spirit
splat_origin:
  key: dnd2024:reference/pages/088
splat_sources:
- title: SRD 5.2.1
  locator: pp. 88
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:feats/epic_boon_feats_boon_of_the_night_spirit
  parameters:
    feat_category: epic_boon
    minimum_character_level: 19
    ability_score_increase:
      choice: any_ability
      amount: 1
      maximum: 30
    merge_with_shadows:
      requires_light:
      - dim
      - darkness
      action: bonus_action
      condition: invisible
      ends_immediately_after_subsequent:
      - action
      - bonus_action
      - reaction
    shadowy_form:
      requires_light:
      - dim
      - darkness
      damage_resistance:
        all_except:
        - psychic
        - radiant
  implementation: dnd2024:feats_epic_boon_feats_boon_of_the_night_spirit
splat_contracts:
- id: dnd2024:feats_epic_boon_feats_boon_of_the_night_spirit
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
  - conditions
  events:
  - character.feat_selected
  - action.bonus
  - action.completed
  - damage.pending
  capabilities:
  - feats
  - ability_scores
  - conditions
  - resistances
splat_conversion:
  status: specified
  category: feat
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 88
  engine_gaps: &id001
  - dnd2024:feats_epic_boon_feats_boon_of_the_night_spirit
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 4034e0b3a677c0ddb04c34110ff43eee4254466e5fc9df0443db35a289148fa3
  parent_name: Epic Boon Feats
  line_span:
    first: 7620
    count: 13
splat_clauses:
- key: source_rules
  text: '# Boon of the Night Spirit Epic Boon Feat (Prerequisite: Level 19+) You gain the following benefits.
    Ability Score Increase. Increase one ability score of your choice by 1, to a maximum of 30. Merge
    with Shadows. While within Dim Light or Darkness, you can give yourself the Invisible condition as
    a Bonus Action. The condition ends on you immediately after you take an action, a Bonus Action, or
    a Reaction. Shadowy Form. While within Dim Light or Darkness, you have Resistance to all damage except
    Psychic and Radiant.'
  sha256: 7864bc5e5475645806e2901281c9b41ad74de8e96a4a536992cc36e15062eb39
  status: contract_specified
splat_requires_mechanics: *id001
---

# Boon of the Night Spirit

<!-- SRD page 88 -->

Epic Boon Feat (Prerequisite: Level 19+)\
You gain the following benefits.\
Ability Score Increase. Increase one ability score\
of your choice by 1, to a maximum of 30.\
Merge with Shadows. While within Dim Light or\
Darkness, you can give yourself the Invisible condi-\
tion as a Bonus Action. The condition ends on you\
immediately after you take an action, a Bonus Ac-\
tion, or a Reaction.\
Shadowy Form. While within Dim Light or Dark-\
ness, you have Resistance to all damage except Psy-\
chic and Radiant.\
