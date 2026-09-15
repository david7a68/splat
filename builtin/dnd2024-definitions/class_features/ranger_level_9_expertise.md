---
splat_schema: 1
splat_id: 396730dc-1242-5fec-a91f-c27d1d67b7c2
splat_kind: rule
splat_key: dnd2024:class_features/ranger_level_9_expertise
splat_name: 'Level 9: Expertise'
splat_origin:
  key: dnd2024:reference/pages/059
splat_sources:
- title: SRD 5.2.1
  locator: pp. 59
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/ranger_level_9_expertise
  parameters:
    expertise:
      count: 2
      eligible:
        proficient: true
        expertise: false
      grant: expertise
    class_scope:
      class: ranger
      minimum_level: 9
  implementation: dnd2024:class_features_ranger_level_9_expertise
splat_contracts:
- id: dnd2024:class_features_ranger_level_9_expertise
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
  - dnd2024:class_features_ranger_level_9_expertise
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 51ac4f16211545380fc86f1da35f4edc42993bcce20a9f78f522a87c31173efc
  parent_name: Ranger
  line_span:
    first: 4971
    count: 4
splat_clauses:
- key: source_rules
  text: '# Level 9: Expertise Choose two of your skill proficiencies with which you lack Expertise. You
    gain Expertise in those skills.'
  sha256: daa93263af3028c4a0253ffd5c5a283375cf8dfca1d0cc2f63296c9491fd415c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 9: Expertise

<!-- SRD page 59 -->

Choose two of your skill proficiencies with which\
you lack Expertise. You gain Expertise in those\
skills.\
