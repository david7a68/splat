---
splat_schema: 1
splat_id: 2b24f1ab-a317-5e70-8e90-e8de54d7e1d6
splat_kind: rule_group
splat_key: dnd2024:backgrounds/soldier
splat_name: Soldier
splat_origin:
  key: dnd2024:reference/pages/083
splat_sources:
- title: SRD 5.2.1
  locator: pp. 83
  url: https://www.dndbeyond.com/srd
splat_rule_group:
  category: background
splat_rule:
  mechanic: dnd2024:backgrounds/soldier
  implementation: dnd2024:backgrounds_soldier
  parameters:
    ability_score_choices:
    - strength
    - dexterity
    - constitution
    ability_increase:
      patterns:
      - - 2
        - 1
      - - 1
        - 1
        - 1
      distinct_abilities: true
      maximum_score: 20
    feat:
      key: dnd2024:feats/origin_feats_savage_attacker
    skill_proficiencies:
    - athletics
    - intimidation
    tool_proficiency:
      count: 1
      category: gaming_set
      selection: actor_choice
    equipment:
      choose: 1
      options:
        equipment:
          items:
          - definition:
              key: dnd2024:equipment/spear
            quantity: 1
          - definition:
              key: dnd2024:equipment/shortbow
            quantity: 1
          - definition:
              key: dnd2024:equipment/ammunition_varies
            quantity: 20
          - definition:
              key: dnd2024:equipment/healer_s_kit_5_gp
            quantity: 1
          - definition:
              key: dnd2024:equipment/quiver_1_gp
            quantity: 1
          - definition:
              key: dnd2024:equipment/clothes_traveler_s_2_gp
            quantity: 1
          currency:
            gp: 14
          ammunition_variant: arrows
          matching_proficient_gaming_set:
            quantity: 1
        currency:
          gp: 50
splat_contracts:
- id: dnd2024:backgrounds_soldier
  inputs:
    actor:
      type: participant
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - ability_increases
  - proficiencies
  - feat_grant
  - equipment_choices
  events:
  - character.background_selected
  capabilities:
  - character_features
  - proficiencies
  - equipment
  - healing
splat_conversion:
  status: specified
  category: background
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 83
  engine_gaps: &id001
  - dnd2024:backgrounds_soldier
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 1c2290a5f4d91819e14fee868475fc4898dd4aa5befd29faf5032f337757a9fe
  line_span:
    first: 7111
    count: 9
splat_clauses:
- key: source_rules
  text: '# Soldier Ability Scores: Strength, Dexterity, Constitution Feat: Savage Attacker (see “Feats”)
    Skill Proficiencies: Athletics and Intimidation Tool Proficiency: Choose one kind of Gaming Set (see
    “Equipment”) Equipment: Choose A or B: (A) Spear, Shortbow, 20 Arrows, Gaming Set (same as above),
    Healer’s Kit, Quiver, Traveler’s Clothes, 14 GP; or (B) 50 GP'
  sha256: 1be789a04fe35dc8e70aaf8c3a95a22f04ec14d131b1aa46a3a9280f3970f3ac
  status: contract_specified
splat_requires_mechanics: *id001
---

# Soldier

<!-- SRD page 83 -->

Ability Scores: Strength, Dexterity, Constitution\
Feat: Savage Attacker (see “Feats”)\
Skill Proficiencies: Athletics and Intimidation\
Tool Proficiency: Choose one kind of Gaming Set (see\
“Equipment”)\
Equipment: Choose A or B: (A) Spear, Shortbow, 20\
Arrows, Gaming Set (same as above), Healer’s Kit,\
Quiver, Traveler’s Clothes, 14 GP; or (B) 50 GP\
