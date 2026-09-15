---
splat_schema: 1
splat_id: f7a92c05-ad3a-51ee-8ee8-a47c492ddaeb
splat_kind: rule
splat_key: dnd2024:feats/epic_boon_feats_boon_of_spell_recall
splat_name: Boon of Spell Recall
splat_origin:
  key: dnd2024:reference/pages/088
splat_sources:
- title: SRD 5.2.1
  locator: pp. 88
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:feats/epic_boon_feats_boon_of_spell_recall
  parameters:
    feat_category: epic_boon
    prerequisites:
      minimum_character_level: 19
      feature: spellcasting
    ability_increase:
      count: 1
      amount: 1
      choices:
      - intelligence
      - wisdom
      - charisma
      maximum_score: 30
    free_casting:
      trigger: actor.casts_spell_with_slot
      slot_levels:
      - 1
      - 2
      - 3
      - 4
      roll:
        count: 1
        sides: 4
      if:
        roll_equals: slot.level
      then:
        slot_expended: false
  implementation: dnd2024:feats_epic_boon_feats_boon_of_spell_recall
splat_contracts:
- id: dnd2024:feats_epic_boon_feats_boon_of_spell_recall
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
  - spell.cast
  capabilities:
  - character_features
  - spellcasting
splat_conversion:
  status: specified
  category: feat
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 88
  engine_gaps: &id001
  - dnd2024:feats_epic_boon_feats_boon_of_spell_recall
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 9a3ec14ce53968dace415e1d6fe51afe0152184ac34c7678c462c2e3e2ee075e
  parent_name: Epic Boon Feats
  line_span:
    first: 7610
    count: 10
splat_clauses:
- key: source_rules
  text: '# Boon of Spell Recall Epic Boon Feat (Prerequisite: Level 19+, Spellcasting Feature) You gain
    the following benefits. Ability Score Increase. Increase your Intelligence, Wisdom, or Charisma score
    by 1, to a maximum of 30. Free Casting. Whenever you cast a spell with a level 1–4 spell slot, roll
    1d4. If the number you roll is the same as the slot’s level, the slot isn’t expended.'
  sha256: f1c73b7389e0545023accbcc0afa5edb4b92094dd2fc06c413d587f149fb9f07
  status: contract_specified
splat_requires_mechanics: *id001
---

# Boon of Spell Recall

<!-- SRD page 88 -->

Epic Boon Feat (Prerequisite: Level 19+, Spellcasting\
Feature)\
You gain the following benefits.\
Ability Score Increase. Increase your Intelligence,\
Wisdom, or Charisma score by 1, to a maximum of\
30.\
Free Casting. Whenever you cast a spell with a\
level 1–4 spell slot, roll 1d4. If the number you roll is\
the same as the slot’s level, the slot isn’t expended.\
