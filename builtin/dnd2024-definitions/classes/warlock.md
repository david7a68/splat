---
splat_schema: 1
splat_id: b2637fe0-2148-5654-b2c3-916cac461834
splat_kind: rule_group
splat_key: dnd2024:classes/warlock
splat_name: Warlock
splat_origin:
  key: dnd2024:reference/pages/070
splat_sources:
- title: SRD 5.2.1
  locator: pp. 70
  url: https://www.dndbeyond.com/srd
splat_rule_group:
  category: class
splat_rule:
  mechanic: dnd2024:classes/warlock
  implementation: dnd2024:classes_warlock
  parameters:
    class: warlock
    primary_abilities:
    - charisma
    hit_point_die:
      count_per_class_level: 1
      sides: 8
    first_class_traits:
      save_proficiencies:
      - wisdom
      - charisma
      skill_proficiencies:
        count: 2
        choices:
        - arcana
        - deception
        - history
        - intimidation
        - investigation
        - nature
        - religion
      weapon_proficiencies:
        simple: true
      armor_training:
      - light
    starting_equipment:
      choose: 1
      options:
        equipment:
          items:
          - definition:
              key: dnd2024:equipment/leather_armor
            quantity: 1
          - definition:
              key: dnd2024:equipment/sickle
            quantity: 1
          - definition:
              key: dnd2024:equipment/dagger
            quantity: 2
          - definition:
              key: dnd2024:equipment/arcane_focus_varies
            quantity: 1
            variant: orb
          - definition:
              key: dnd2024:equipment/book_25_gp
            quantity: 1
            variant: occult_lore
          - definition:
              key: dnd2024:equipment/scholar_s_pack_40_gp
            quantity: 1
          currency:
            gp: 15
        currency:
          currency:
            gp: 100
    progression:
      key: dnd2024:tables/warlock_progression
    feature_grants:
    - minimum_class_level: 11
      definition:
        key: dnd2024:class_features/warlock_level_11_mystic_arcanum
    - minimum_class_level: 19
      definition:
        key: dnd2024:class_features/warlock_level_19_epic_boon
    - minimum_class_level: 1
      definition:
        key: dnd2024:class_features/warlock_level_1_eldritch_invocations
    - minimum_class_level: 1
      definition:
        key: dnd2024:class_features/warlock_level_1_pact_magic
    - minimum_class_level: 20
      definition:
        key: dnd2024:class_features/warlock_level_20_eldritch_master
    - minimum_class_level: 2
      definition:
        key: dnd2024:class_features/warlock_level_2_magical_cunning
    - minimum_class_level: 3
      definition:
        key: dnd2024:class_features/warlock_level_3_warlock_subclass
    - minimum_class_level: 4
      definition:
        key: dnd2024:class_features/warlock_level_4_ability_score_improvement
    - minimum_class_level: 9
      definition:
        key: dnd2024:class_features/warlock_level_9_contact_patron
splat_contracts:
- id: dnd2024:classes_warlock
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
  - proficiencies
  - d20_tests
  - equipment
splat_conversion:
  status: specified
  category: class
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 70
  engine_gaps: &id001
  - dnd2024:classes_warlock
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 916755b43bba755d8b2fe91023f350496a7b00d2d067ee5d06f8ed757b9d9a9d
  line_span:
    first: 5933
    count: 16
splat_clauses:
- key: source_rules
  text: '# Warlock Core Warlock Traits Primary Ability Charisma Hit Point Die D8 per Warlock level Saving
    Throw Wisdom and Charisma Proficiencies Skill Proficiencies Choose 2: Arcana, Deception, History,
    Intimidation, Investigation, Nature, or Religion Weapon Proficiencies Simple weapons Armor Training
    Light armor Starting Equipment Choose A or B: (A) Leather Armor, Sickle, 2 Daggers, Arcane Focus (orb),
    Book (occult lore), Scholar’s Pack, and 15 GP; or (B) 100 GP'
  sha256: f8d7caf3d42103cf03a29fe0a4fd14b2b6048ff96c700f661602aa711f552b84
  status: contract_specified
splat_requires_mechanics: *id001
---

# Warlock

<!-- SRD page 70 -->

Core Warlock Traits\
Primary Ability Charisma\
Hit Point Die D8 per Warlock level\
Saving Throw Wisdom and Charisma\
Proficiencies\
Skill Proficiencies Choose 2: Arcana, Deception,\
History, Intimidation, Investi-\
gation, Nature, or Religion\
Weapon Proficiencies Simple weapons\
Armor Training Light armor\
Starting Equipment Choose A or B: (A) Leather\
Armor, Sickle, 2 Daggers, Ar-\
cane Focus (orb), Book (occult\
lore), Scholar’s Pack, and 15\
GP; or (B) 100 GP\
