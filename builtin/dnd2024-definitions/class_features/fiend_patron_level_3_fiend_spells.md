---
splat_schema: 1
splat_id: 12d4ee93-4ac1-51e8-ae82-b2f07e20f448
splat_kind: rule
splat_key: dnd2024:class_features/fiend_patron_level_3_fiend_spells
splat_name: 'Level 3: Fiend Spells'
splat_origin:
  key: dnd2024:reference/pages/076
splat_sources:
- title: SRD 5.2.1
  locator: pp. 76
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/fiend_patron_level_3_fiend_spells
  parameters:
    class: warlock
    always_prepared:
    - minimum_class_level: 3
      spells:
      - key: dnd2024:spells/burning_hands
      - key: dnd2024:spells/command
      - key: dnd2024:spells/scorching_ray
      - key: dnd2024:spells/suggestion
    - minimum_class_level: 5
      spells:
      - key: dnd2024:spells/fireball
      - key: dnd2024:spells/stinking_cloud
    - minimum_class_level: 7
      spells:
      - key: dnd2024:spells/fire_shield
      - key: dnd2024:spells/wall_of_fire
    - minimum_class_level: 9
      spells:
      - key: dnd2024:spells/geas
      - key: dnd2024:spells/insect_plague
    include_lower_levels: true
    counts_against_prepared_limit: false
    class_scope:
      class: warlock
      minimum_level: 3
  implementation: dnd2024:class_features_fiend_patron_level_3_fiend_spells
splat_contracts:
- id: dnd2024:class_features_fiend_patron_level_3_fiend_spells
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
  - 76
  engine_gaps: &id001
  - dnd2024:class_features_fiend_patron_level_3_fiend_spells
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 29f32cf1bbcd52a9ac79a6c8229adaaef2f4d77afb261e42834fa27802288301
  parent_name: Fiend Patron
  line_span:
    first: 6487
    count: 12
splat_clauses:
- key: source_rules
  text: '# Level 3: Fiend Spells The magic of your patron ensures you always have certain spells ready;
    when you reach a Warlock level specified in the Fiend Spells table, you thereafter always have the
    listed spells prepared. Fiend Spells Warlock Level Spells 3 Burning Hands, Command, Scorching Ray,
    Suggestion 5 Fireball, Stinking Cloud 7 Fire Shield, Wall of Fire 9 Geas, Insect Plague'
  sha256: 1ccc23c39938a23895eca5d9d190dddf4cc2c94722b72012d8e6518d88ecf8f8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 3: Fiend Spells

<!-- SRD page 76 -->

The magic of your patron ensures you always have\
certain spells ready; when you reach a Warlock\
level specified in the Fiend Spells table, you thereaf-\
ter always have the listed spells prepared.\
Fiend Spells\
Warlock Level Spells\
3 Burning Hands, Command,\
Scorching Ray, Suggestion\
5 Fireball, Stinking Cloud\
7 Fire Shield, Wall of Fire\
9 Geas, Insect Plague\
