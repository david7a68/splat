---
splat_schema: 1
splat_id: c13c443b-6460-53bf-bc48-acdb876af0d9
splat_kind: rule
splat_key: dnd2024:class_features/cleric_level_1_divine_order
splat_name: 'Level 1: Divine Order'
splat_origin:
  key: dnd2024:reference/pages/037
splat_sources:
- title: SRD 5.2.1
  locator: pp. 37
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/cleric_level_1_divine_order
  parameters:
    choice:
      count: 1
      options:
      - id: protector
        weapon_proficiencies:
        - martial
        armor_training:
        - heavy
      - id: thaumaturge
        cantrip_choice:
          list: cleric
          count: 1
        check_bonus:
          ability: intelligence
          skills:
          - arcana
          - religion
          amount:
            op: max
            args:
            - 1
            - stat: abilities.wisdom.modifier
  implementation: dnd2024:class_features_cleric_level_1_divine_order
splat_contracts:
- id: dnd2024:class_features_cleric_level_1_divine_order
  inputs:
    actor:
      type: participant
      required: true
    selections:
      required: false
      type: choice
  outputs:
  - feature_state
  - choices
  - spell_grants
  - proficiencies
  events:
  - character.feature_gained
  capabilities:
  - character_choices
  - proficiencies
  - spellcasting
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 37
  engine_gaps: &id001
  - dnd2024:class_features_cleric_level_1_divine_order
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: c56c188d2b54073d3f4ba6641646d237749eb54b43f462e86f2d0d56b3b70799
  parent_name: Cleric
  line_span:
    first: 3039
    count: 11
splat_clauses:
- key: source_rules
  text: '# Level 1: Divine Order You have dedicated yourself to one of the following sacred roles of your
    choice. Protector. Trained for battle, you gain proficiency with Martial weapons and training with
    Heavy armor. Thaumaturge. You know one extra cantrip from the Cleric spell list. In addition, your
    mystical connection to the divine gives you a bonus to your Intelligence (Arcana or Religion) checks.
    The bonus equals your Wisdom modifier (minimum of +1).'
  sha256: f192d01c0b6427572c33bdfdbf4998f3167825cc610f4d1fe0a7d077fe27debf
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 1: Divine Order

<!-- SRD page 37 -->

You have dedicated yourself to one of the following\
sacred roles of your choice.\
Protector. Trained for battle, you gain proficiency\
with Martial weapons and training with Heavy\
armor.\
Thaumaturge. You know one extra cantrip from\
the Cleric spell list. In addition, your mystical con-\
nection to the divine gives you a bonus to your\
Intelligence (Arcana or Religion) checks. The bonus\
equals your Wisdom modifier (minimum of +1).\
