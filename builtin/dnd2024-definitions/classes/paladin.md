---
splat_schema: 1
splat_id: f4823d01-8fa3-5e60-9f72-1af3248f7d9f
splat_kind: rule_group
splat_key: dnd2024:classes/paladin
splat_name: Paladin
splat_origin:
  key: dnd2024:reference/pages/053
splat_sources:
- title: SRD 5.2.1
  locator: pp. 53
  url: https://www.dndbeyond.com/srd
splat_rule_group:
  category: class
splat_rule:
  mechanic: dnd2024:classes/paladin
  implementation: dnd2024:classes_paladin
  parameters:
    class: paladin
    primary_abilities:
    - strength
    - charisma
    hit_point_die:
      count_per_class_level: 1
      sides: 10
    first_class_traits:
      save_proficiencies:
      - wisdom
      - charisma
      skill_proficiencies:
        count: 2
        choices:
        - athletics
        - insight
        - intimidation
        - medicine
        - persuasion
        - religion
      weapon_proficiencies:
        simple: true
        martial: true
      armor_training:
      - light
      - medium
      - heavy
      - shield
    starting_equipment:
      choose: 1
      options:
        equipment:
          items:
          - definition:
              key: dnd2024:equipment/chain_mail
            quantity: 1
          - definition:
              key: dnd2024:equipment/shield
            quantity: 1
          - definition:
              key: dnd2024:equipment/longsword
            quantity: 1
          - definition:
              key: dnd2024:equipment/javelin
            quantity: 6
          - definition:
              key: dnd2024:equipment/holy_symbol_varies
            quantity: 1
          - definition:
              key: dnd2024:equipment/priest_s_pack_33_gp
            quantity: 1
          currency:
            gp: 9
        currency:
          currency:
            gp: 150
    progression:
      key: dnd2024:tables/paladin_progression
    feature_grants:
    - minimum_class_level: 10
      definition:
        key: dnd2024:class_features/paladin_level_10_aura_of_courage
    - minimum_class_level: 11
      definition:
        key: dnd2024:class_features/paladin_level_11_radiant_strikes
    - minimum_class_level: 14
      definition:
        key: dnd2024:class_features/paladin_level_14_restoring_touch
    - minimum_class_level: 18
      definition:
        key: dnd2024:class_features/paladin_level_18_aura_expansion
    - minimum_class_level: 19
      definition:
        key: dnd2024:class_features/paladin_level_19_epic_boon
    - minimum_class_level: 1
      definition:
        key: dnd2024:class_features/paladin_level_1_lay_on_hands
    - minimum_class_level: 1
      definition:
        key: dnd2024:class_features/paladin_level_1_spellcasting
    - minimum_class_level: 1
      definition:
        key: dnd2024:class_features/paladin_level_1_weapon_mastery
    - minimum_class_level: 2
      definition:
        key: dnd2024:class_features/paladin_level_2_fighting_style
    - minimum_class_level: 2
      definition:
        key: dnd2024:class_features/paladin_level_2_paladin_s_smite
    - minimum_class_level: 3
      definition:
        key: dnd2024:class_features/paladin_level_3_channel_divinity
    - minimum_class_level: 3
      definition:
        key: dnd2024:class_features/paladin_level_3_paladin_subclass
    - minimum_class_level: 4
      definition:
        key: dnd2024:class_features/paladin_level_4_ability_score_improvement
    - minimum_class_level: 5
      definition:
        key: dnd2024:class_features/paladin_level_5_extra_attack
    - minimum_class_level: 5
      definition:
        key: dnd2024:class_features/paladin_level_5_faithful_steed
    - minimum_class_level: 6
      definition:
        key: dnd2024:class_features/paladin_level_6_aura_of_protection
    - minimum_class_level: 9
      definition:
        key: dnd2024:class_features/paladin_level_9_abjure_foes
splat_contracts:
- id: dnd2024:classes_paladin
  inputs:
    actor:
      type: participant
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - class_traits
  - equipment_choices
  - progression
  - feature_grants
  events:
  - character.class_selected
  - character.level_gained
  capabilities:
  - character_features
  - spellcasting
  - proficiencies
  - d20_tests
  - equipment
  - auras
splat_conversion:
  status: specified
  category: class
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 53
  engine_gaps: &id001
  - dnd2024:classes_paladin
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: c610c89266a606118ac56e9343a2c2c1e87086f67e0bc10b65121e75a7f3458a
  line_span:
    first: 4398
    count: 16
splat_clauses:
- key: source_rules
  text: '# Paladin Core Paladin Traits Primary Ability Strength and Charisma Hit Point Die D10 per Paladin
    level Saving Throw Wisdom and Charisma Proficiencies Skill Proficiencies Choose 2: Athletics, Insight,
    Intimidation, Medicine, Persuasion, or Religion Weapon Proficiencies Simple and Martial weapons Armor
    Training Light, Medium, and Heavy armor and Shields Starting Equipment Choose A or B: (A) Chain Mail,
    Shield, Longsword, 6 Javelins, Holy Symbol, Priest’s Pack, and 9 GP; or (B) 150 GP'
  sha256: 35132eb8b2b0837569860812551f8f8a5143a253c7faaa5b2a76d907e6889496
  status: contract_specified
splat_requires_mechanics: *id001
---

# Paladin

<!-- SRD page 53 -->

Core Paladin Traits\
Primary Ability Strength and Charisma\
Hit Point Die D10 per Paladin level\
Saving Throw Wisdom and Charisma\
Proficiencies\
Skill Proficiencies Choose 2: Athletics, Insight,\
Intimidation, Medicine,\
Persuasion, or Religion\
Weapon Proficiencies Simple and Martial weapons\
Armor Training Light, Medium, and Heavy ar-\
mor and Shields\
Starting Equipment Choose A or B: (A) Chain Mail,\
Shield, Longsword, 6 Javelins,\
Holy Symbol, Priest’s Pack,\
and 9 GP; or (B) 150 GP\
