---
splat_schema: 1
splat_id: 697788ce-3b3f-5d8c-959b-61491a3eb620
splat_kind: rule_group
splat_key: dnd2024:backgrounds/sage
splat_name: Sage
splat_origin:
  key: dnd2024:reference/pages/083
splat_sources:
- title: SRD 5.2.1
  locator: pp. 83
  url: https://www.dndbeyond.com/srd
splat_rule_group:
  category: background
splat_rule:
  mechanic: dnd2024:backgrounds/sage
  implementation: dnd2024:backgrounds_sage
  parameters:
    ability_score_choices:
      eligible:
      - constitution
      - intelligence
      - wisdom
      allocations:
      - - 2
        - 1
      - - 1
        - 1
        - 1
      distinct_abilities: true
      maximum_score: 20
    feat_grant:
      definition:
        key: dnd2024:feats/origin_feats_magic_initiate
      spell_list: wizard
    skill_proficiencies:
    - arcana
    - history
    tool_proficiencies:
    - calligraphers_supplies
    equipment_choice:
      count: 1
      options:
      - id: a
        grants:
        - item_type: quarterstaff
          quantity: 1
        - item_type: calligraphers_supplies
          quantity: 1
        - item_type: book_history
          quantity: 1
        - item_type: parchment_sheet
          quantity: 8
        - item_type: robe
          quantity: 1
        - item_type: gp
          quantity: 8
      - id: b
        grants:
        - item_type: gp
          quantity: 50
splat_contracts:
- id: dnd2024:backgrounds_sage
  inputs:
    actor:
      type: participant
      required: true
    selections:
      required: false
      type: choice
  outputs:
  - feature_state
  - proficiencies
  - inventory_changes
  events:
  - character.background_selected
  capabilities:
  - character_creation
  - ability_scores
  - proficiencies
  - feats
  - inventory
splat_conversion:
  status: specified
  category: background
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 83
  engine_gaps: &id001
  - dnd2024:backgrounds_sage
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 0d57ba18f810866c341bf1099c99840d04e0677fb6caa6b64bfc9c00b14d446e
  line_span:
    first: 7103
    count: 8
splat_clauses:
- key: source_rules
  text: '# Sage Ability Scores: Constitution, Intelligence, Wisdom Feat: Magic Initiate (Wizard) (see
    “Feats”) Skill Proficiencies: Arcana and History Tool Proficiency: Calligrapher’s Supplies Equipment:
    Choose A or B: (A) Quarterstaff, Calligrapher’s Supplies, Book (history), Parchment (8 sheets), Robe,
    8 GP; or (B) 50 GP'
  sha256: dcdc7faa1ce52c404b2c415336023ae7b091c1261651ee84d701fcb056edcb45
  status: contract_specified
splat_requires_mechanics: *id001
---

# Sage

<!-- SRD page 83 -->

Ability Scores: Constitution, Intelligence, Wisdom\
Feat: Magic Initiate (Wizard) (see “Feats”)\
Skill Proficiencies: Arcana and History\
Tool Proficiency: Calligrapher’s Supplies\
Equipment: Choose A or B: (A) Quarterstaff, Callig-\
rapher’s Supplies, Book (history), Parchment (8\
sheets), Robe, 8 GP; or (B) 50 GP\
