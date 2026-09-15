---
splat_schema: 1
splat_id: dff89ced-250c-5c88-a4d6-74a0e18c4ef9
splat_kind: rule
splat_key: dnd2024:rules/druid_as_a_multiclass_character
splat_name: As a Multiclass Character
splat_origin:
  key: dnd2024:reference/pages/041
splat_sources:
- title: SRD 5.2.1
  locator: pp. 41
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/druid_as_a_multiclass_character
  parameters:
    class:
      key: dnd2024:classes/druid
    entry: multiclass
    hit_die_sides: 8
    armor_training:
    - light
    - shield
    features:
    - level: 1
      feature:
        key: dnd2024:class_features/druid_level_1_druidic
    - level: 1
      feature:
        key: dnd2024:class_features/druid_level_1_primal_order
    - level: 1
      feature:
        key: dnd2024:class_features/druid_level_1_spellcasting
    spell_slots_rule:
      key: dnd2024:rules/multiclassing_spellcasting
  implementation: dnd2024:rules_druid_as_a_multiclass_character
splat_contracts:
- id: dnd2024:rules_druid_as_a_multiclass_character
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - multiclass_grants
  events: []
  capabilities:
  - multiclassing
  - character_features
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 41
  engine_gaps: &id001
  - dnd2024:rules_druid_as_a_multiclass_character
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: bce9c46a07becb2ac4e199ad3476028f9489e1331b58c10ae4dec4a5b9cdddea
  parent_name: Druid
  line_span:
    first: 3388
    count: 8
splat_clauses:
- key: source_rules
  text: '# As a Multiclass Character • Gain the following traits from the Core Druid Traits table: Hit
    Point Die and training with Light armor and Shields. • Gain the Druid’s level 1 features, which are
    listed in the Druid Features table. See the multiclassing rules in “Character Creation” to determine
    your available spell slots.'
  sha256: 035754baeae16d9b58a8434c501a5eeeb95957dfbf71f14510dcd9688f9d33b2
  status: contract_specified
splat_requires_mechanics: *id001
---

# As a Multiclass Character

<!-- SRD page 41 -->

•  Gain the following traits from the Core Druid\
Traits table: Hit Point Die and training with Light\
armor and Shields.\
•  Gain the Druid’s level 1 features, which are listed\
in the Druid Features table. See the multiclassing\
rules in “Character Creation” to determine your\
available spell slots.\
