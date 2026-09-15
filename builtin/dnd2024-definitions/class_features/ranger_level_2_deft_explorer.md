---
splat_schema: 1
splat_id: 0949c6f4-683b-58b9-bcd3-d9ed5eb62ff7
splat_kind: rule
splat_key: dnd2024:class_features/ranger_level_2_deft_explorer
splat_name: 'Level 2: Deft Explorer'
splat_origin:
  key: dnd2024:reference/pages/059
splat_sources:
- title: SRD 5.2.1
  locator: pp. 59
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/ranger_level_2_deft_explorer
  parameters:
    expertise:
      count: 1
      eligible:
        proficient: true
        expertise: false
      grant: expertise
    languages:
      count: 2
      selection: actor_choice
      eligible_tables:
      - standard_languages
      - rare_languages
    class_scope:
      class: ranger
      minimum_level: 2
  implementation: dnd2024:class_features_ranger_level_2_deft_explorer
splat_contracts:
- id: dnd2024:class_features_ranger_level_2_deft_explorer
  inputs:
    actor:
      type: participant
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - character.feature_acquired
  capabilities:
  - character_features
  - proficiencies
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 59
  engine_gaps: &id001
  - dnd2024:class_features_ranger_level_2_deft_explorer
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: f3a31e5e44e08c7852757232c97117bafcf6eec2bf87225f6274b26e487d0e30
  parent_name: Ranger
  line_span:
    first: 4931
    count: 9
splat_clauses:
- key: source_rules
  text: '# Level 2: Deft Explorer Thanks to your travels, you gain the following benefits. Expertise.
    Choose one of your skill proficiencies with which you lack Expertise. You gain Expertise in that skill.
    Languages. You know two languages of your choice from the language tables in “Character Creation.”'
  sha256: 3f71bb29a3b88098f1d340d31d9740bb867e0dcb18188b330d664fb66ea1393a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 2: Deft Explorer

<!-- SRD page 59 -->

Thanks to your travels, you gain the following\
benefits.\
Expertise. Choose one of your skill proficiencies\
with which you lack Expertise. You gain Expertise\
in that skill.\
Languages. You know two languages of your\
choice from the language tables in “Character\
Creation.”\
