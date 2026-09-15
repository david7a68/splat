---
splat_schema: 1
splat_id: c69937e7-0b05-56d7-a464-f13afb2a5542
splat_kind: rule_group
splat_key: dnd2024:classes/wizard
splat_name: Wizard
splat_origin:
  key: dnd2024:reference/pages/077
splat_sources:
- title: SRD 5.2.1
  locator: pp. 77
  url: https://www.dndbeyond.com/srd
splat_rule_group:
  category: class
splat_rule:
  mechanic: dnd2024:classes/wizard
  implementation: dnd2024:classes_wizard
  parameters:
    class: wizard
    primary_abilities:
    - intelligence
    hit_point_die:
      count_per_class_level: 1
      sides: 6
    first_class_traits:
      save_proficiencies:
      - intelligence
      - wisdom
      skill_proficiencies:
        count: 2
        choices:
        - arcana
        - history
        - insight
        - investigation
        - medicine
        - nature
        - religion
      weapon_proficiencies:
        simple: true
      armor_training: []
    starting_equipment:
      choose: 1
      options:
        equipment:
          items:
          - definition:
              key: dnd2024:equipment/dagger
            quantity: 2
          - definition:
              key: dnd2024:equipment/arcane_focus_varies
            quantity: 1
            variant: quarterstaff
          - definition:
              key: dnd2024:equipment/robe_1_gp
            quantity: 1
          - definition:
              key: dnd2024:equipment/scholar_s_pack_40_gp
            quantity: 1
          currency:
            gp: 5
          additional_items:
          - kind: spellbook
            quantity: 1
        currency:
          currency:
            gp: 55
    progression:
      key: dnd2024:tables/wizard_progression
    feature_grants:
    - minimum_class_level: 18
      definition:
        key: dnd2024:class_features/wizard_level_18_spell_mastery
    - minimum_class_level: 19
      definition:
        key: dnd2024:class_features/wizard_level_19_epic_boon
    - minimum_class_level: 1
      definition:
        key: dnd2024:class_features/wizard_level_1_arcane_recovery
    - minimum_class_level: 1
      definition:
        key: dnd2024:class_features/wizard_level_1_ritual_adept
    - minimum_class_level: 1
      definition:
        key: dnd2024:class_features/wizard_level_1_spellcasting
    - minimum_class_level: 20
      definition:
        key: dnd2024:class_features/wizard_level_20_signature_spells
    - minimum_class_level: 2
      definition:
        key: dnd2024:class_features/wizard_level_2_scholar
    - minimum_class_level: 3
      definition:
        key: dnd2024:class_features/wizard_level_3_wizard_subclass
    - minimum_class_level: 4
      definition:
        key: dnd2024:class_features/wizard_level_4_ability_score_improvement
    - minimum_class_level: 5
      definition:
        key: dnd2024:class_features/wizard_level_5_memorize_spell
splat_contracts:
- id: dnd2024:classes_wizard
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
splat_conversion:
  status: specified
  category: class
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 77
  engine_gaps: &id001
  - dnd2024:classes_wizard
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 02f4648e3a94a96a45da890bedbff79659afab3c57c0233ca3c7c35df4099865
  line_span:
    first: 6529
    count: 15
splat_clauses:
- key: source_rules
  text: '# Wizard Core Wizard Traits Primary Ability Intelligence Hit Point Die D6 per Wizard level Saving
    Throw Intelligence and Wisdom Proficiencies Skill Proficiencies Choose 2: Arcana, History, Insight,
    Investigation, Medicine, Nature, or Religion Weapon Proficiencies Simple weapons Armor Training None
    Starting Equipment Choose A or B: (A) 2 Daggers, Arcane Focus (Quarterstaff), Robe, Spellbook, Scholar’s
    Pack, and 5 GP; or (B) 55 GP'
  sha256: 4f591ca3a223a298d30574cd303ba24a2d716ecb73f3782401324c18a8a88231
  status: contract_specified
splat_requires_mechanics: *id001
---

# Wizard

<!-- SRD page 77 -->

Core Wizard Traits\
Primary Ability Intelligence\
Hit Point Die D6 per Wizard level\
Saving Throw Intelligence and Wisdom\
Proficiencies\
Skill Proficiencies Choose 2: Arcana, History, In-\
sight, Investigation, Medicine,\
Nature, or Religion\
Weapon Proficiencies Simple weapons\
Armor Training None\
Starting Equipment Choose A or B: (A) 2 Daggers,\
Arcane Focus (Quarterstaff),\
Robe, Spellbook, Scholar’s\
Pack, and 5 GP; or (B) 55 GP\
