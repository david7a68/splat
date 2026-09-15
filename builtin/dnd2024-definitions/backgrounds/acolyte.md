---
splat_schema: 1
splat_id: b21da6c6-a53f-5ffc-82b9-b1bb6fa0acf3
splat_kind: rule_group
splat_key: dnd2024:backgrounds/acolyte
splat_name: Acolyte
splat_origin:
  key: dnd2024:reference/pages/083
splat_sources:
- title: SRD 5.2.1
  locator: pp. 83
  url: https://www.dndbeyond.com/srd
splat_rule_group:
  category: background
splat_rule:
  mechanic: dnd2024:backgrounds/acolyte
  implementation: dnd2024:backgrounds_acolyte
  parameters:
    ability_score_choices:
      eligible:
      - intelligence
      - wisdom
      - charisma
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
      spell_list: cleric
    skill_proficiencies:
    - insight
    - religion
    tool_proficiencies:
    - calligraphers_supplies
    equipment_choice:
      count: 1
      options:
      - id: a
        grants:
        - item_type: calligraphers_supplies
          quantity: 1
        - item_type: book_prayers
          quantity: 1
        - item_type: holy_symbol
          quantity: 1
        - item_type: parchment_sheet
          quantity: 10
        - item_type: robe
          quantity: 1
        - item_type: gp
          quantity: 8
      - id: b
        grants:
        - item_type: gp
          quantity: 50
splat_contracts:
- id: dnd2024:backgrounds_acolyte
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
  - dnd2024:backgrounds_acolyte
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: d3503d8aaf2c8880d1ba07cd64f36bb98c50a4588b8362707e07fc6a238bac21
  line_span:
    first: 7087
    count: 8
splat_clauses:
- key: source_rules
  text: '# Acolyte Ability Scores: Intelligence, Wisdom, Charisma Feat: Magic Initiate (Cleric) (see “Feats”)
    Skill Proficiencies: Insight and Religion Tool Proficiency: Calligrapher’s Supplies Equipment: Choose
    A or B: (A) Calligrapher’s Supplies, Book (prayers), Holy Symbol, Parchment (10 sheets), Robe, 8 GP;
    or (B) 50 GP'
  sha256: 6309d22770a2019cbff81071cc777775b9c6fdc309e49b67a6578e416d18fa01
  status: contract_specified
splat_requires_mechanics: *id001
---

# Acolyte

<!-- SRD page 83 -->

Ability Scores: Intelligence, Wisdom, Charisma\
Feat: Magic Initiate (Cleric) (see “Feats”)\
Skill Proficiencies: Insight and Religion\
Tool Proficiency: Calligrapher’s Supplies\
Equipment: Choose A or B: (A) Calligrapher’s Supplies,\
Book (prayers), Holy Symbol, Parchment (10 sheets),\
Robe, 8 GP; or (B) 50 GP\
