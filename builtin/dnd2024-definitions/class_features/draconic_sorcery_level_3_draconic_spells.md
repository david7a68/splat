---
splat_schema: 1
splat_id: 8c5bc5c2-5e14-5ec3-be06-811ab3553115
splat_kind: rule
splat_key: dnd2024:class_features/draconic_sorcery_level_3_draconic_spells
splat_name: 'Level 3: Draconic Spells'
splat_origin:
  key: dnd2024:reference/pages/070
splat_sources:
- title: SRD 5.2.1
  locator: pp. 70
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/draconic_sorcery_level_3_draconic_spells
  parameters:
    class: sorcerer
    always_prepared:
    - minimum_class_level: 3
      spells:
      - key: dnd2024:spells/alter_self
      - key: dnd2024:spells/chromatic_orb
      - key: dnd2024:spells/command
      - key: dnd2024:spells/dragon_s_breath
    - minimum_class_level: 5
      spells:
      - key: dnd2024:spells/fear
      - key: dnd2024:spells/fly
    - minimum_class_level: 7
      spells:
      - key: dnd2024:spells/arcane_eye
      - key: dnd2024:spells/charm_monster
    - minimum_class_level: 9
      spells:
      - key: dnd2024:spells/legend_lore
      - key: dnd2024:spells/summon_dragon
    include_lower_levels: true
    counts_against_prepared_limit: false
    class_scope:
      class: sorcerer
      minimum_level: 3
  implementation: dnd2024:class_features_draconic_sorcery_level_3_draconic_spells
splat_contracts:
- id: dnd2024:class_features_draconic_sorcery_level_3_draconic_spells
  inputs:
    actor:
      type: participant
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - prepared_spells
  events:
  - character.level_gained
  capabilities:
  - character_features
  - spellcasting
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 70
  engine_gaps: &id001
  - dnd2024:class_features_draconic_sorcery_level_3_draconic_spells
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 857c261688846659ee6f1af51fdbd9d0ec51d431469cb75c3cb3feb653feaaab
  parent_name: Draconic Sorcery
  line_span:
    first: 5895
    count: 12
splat_clauses:
- key: source_rules
  text: '# Level 3: Draconic Spells When you reach a Sorcerer level specified in the Draconic Spells table,
    you thereafter always have the listed spells prepared. Draconic Spells Sorcerer Level Spells 3 Alter
    Self, Chromatic Orb, Command, Dragon’s Breath 5 Fear, Fly 7 Arcane Eye, Charm Monster 9 Legend Lore,
    Summon Dragon'
  sha256: bfd2fb487976cb90104f154b60fb0a1f38e5fb5832e2f2c48e00760d57fa54b5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 3: Draconic Spells

<!-- SRD page 70 -->

When you reach a Sorcerer level specified in the\
Draconic Spells table, you thereafter always have\
the listed spells prepared.\
Draconic Spells\
Sorcerer\
Level Spells\
3 Alter Self, Chromatic Orb, Command,\
Dragon’s Breath\
5 Fear, Fly\
7 Arcane Eye, Charm Monster\
9 Legend Lore, Summon Dragon\
