---
splat_schema: 1
splat_id: b6f3840e-2704-5a6c-bfc9-36632f08459a
splat_kind: rule_group
splat_key: dnd2024:backgrounds/criminal
splat_name: Criminal
splat_origin:
  key: dnd2024:reference/pages/083
splat_sources:
- title: SRD 5.2.1
  locator: pp. 83
  url: https://www.dndbeyond.com/srd
splat_rule_group:
  category: background
splat_rule:
  mechanic: dnd2024:backgrounds/criminal
  implementation: dnd2024:backgrounds_criminal
  parameters:
    ability_score_choices:
    - dexterity
    - constitution
    - intelligence
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
      key: dnd2024:feats/origin_feats_alert
    skill_proficiencies:
    - sleight_of_hand
    - stealth
    tool_proficiency:
      key: dnd2024:equipment/other_tools_thieves_tools_25_gp
    equipment:
      choose: 1
      options:
        equipment:
          items:
          - definition:
              key: dnd2024:equipment/dagger
            quantity: 2
          - definition:
              key: dnd2024:equipment/other_tools_thieves_tools_25_gp
            quantity: 1
          - definition:
              key: dnd2024:equipment/crowbar_2_gp
            quantity: 1
          - definition:
              key: dnd2024:equipment/pouch_5_sp
            quantity: 2
          - definition:
              key: dnd2024:equipment/clothes_traveler_s_2_gp
            quantity: 1
          currency:
            gp: 16
        currency:
          gp: 50
splat_contracts:
- id: dnd2024:backgrounds_criminal
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
splat_conversion:
  status: specified
  category: background
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 83
  engine_gaps: &id001
  - dnd2024:backgrounds_criminal
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 63c308edcd283fe4c664fd6625176e9309f9b102a58348d314fa2c71dbf7e1ac
  line_span:
    first: 7095
    count: 8
splat_clauses:
- key: source_rules
  text: '# Criminal Ability Scores: Dexterity, Constitution, Intelligence Feat: Alert (see “Feats”) Skill
    Proficiencies: Sleight of Hand and Stealth Tool Proficiency: Thieves’ Tools Equipment: Choose A or
    B: (A) 2 Daggers, Thieves’ Tools, Crowbar, 2 Pouches, Traveler’s Clothes, 16 GP; or (B) 50 GP'
  sha256: 030ce48c43df562610f653aec3523c2b133038a9edb12de514ad8b737977bbef
  status: contract_specified
splat_requires_mechanics: *id001
---

# Criminal

<!-- SRD page 83 -->

Ability Scores: Dexterity, Constitution, Intelligence\
Feat: Alert (see “Feats”)\
Skill Proficiencies: Sleight of Hand and Stealth\
Tool Proficiency: Thieves’ Tools\
Equipment: Choose A or B: (A) 2 Daggers, Thieves’\
Tools, Crowbar, 2 Pouches, Traveler’s Clothes,\
16 GP; or (B) 50 GP\
