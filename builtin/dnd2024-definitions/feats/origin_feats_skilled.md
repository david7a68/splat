---
splat_schema: 1
splat_id: 8d40a00d-c470-5f4d-8c8a-89b7ba31e9c5
splat_kind: rule
splat_key: dnd2024:feats/origin_feats_skilled
splat_name: Skilled
splat_origin:
  key: dnd2024:reference/pages/087
splat_sources:
- title: SRD 5.2.1
  locator: pp. 87
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:feats/origin_feats_skilled
  parameters:
    feat_category: origin
    proficiency_choices:
      count: 3
      categories:
      - skill
      - tool
      mix_categories: true
    repeatable: true
  implementation: dnd2024:feats_origin_feats_skilled
splat_contracts:
- id: dnd2024:feats_origin_feats_skilled
  inputs:
    actor:
      type: participant
      required: true
  outputs:
  - feature_state
  - proficiencies
  events:
  - character.feat_selected
  capabilities:
  - feats
  - proficiencies
splat_conversion:
  status: specified
  category: feat
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 87
  engine_gaps: &id001
  - dnd2024:feats_origin_feats_skilled
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 2f97373968f6e66aac32136e6fdcacfcc8a9e1f0cb6027642a01cbc9630b59b9
  parent_name: Origin Feats
  line_span:
    first: 7514
    count: 6
splat_clauses:
- key: source_rules
  text: '# Skilled Origin Feat You gain proficiency in any combination of three skills or tools of your
    choice. Repeatable. You can take this feat more than once.'
  sha256: 7cfeee69d368ff9bcf262c8332278802ff1b59c0a96aef4be9993869ba83d00e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Skilled

<!-- SRD page 87 -->

Origin Feat\
You gain proficiency in any combination of three\
skills or tools of your choice.\
Repeatable. You can take this feat more than\
once.\
