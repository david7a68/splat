---
splat_schema: 1
splat_id: 87d3c1df-8044-53c0-86ec-b1cec9303f8f
splat_kind: rule
splat_key: dnd2024:class_features/bard_level_18_superior_inspiration
splat_name: 'Level 18: Superior Inspiration'
splat_origin:
  key: dnd2024:reference/pages/033
splat_sources:
- title: SRD 5.2.1
  locator: pp. 33
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/bard_level_18_superior_inspiration
  parameters:
    resource: bardic_inspiration
    recover_to_minimum: 2
  implementation: dnd2024:class_features_bard_level_18_superior_inspiration
splat_contracts:
- id: dnd2024:class_features_bard_level_18_superior_inspiration
  inputs:
    actor:
      type: participant
      required: true
  outputs:
  - feature_state
  - resource_changes
  events:
  - initiative.rolled
  capabilities:
  - resources
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 33
  engine_gaps: &id001
  - dnd2024:class_features_bard_level_18_superior_inspiration
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: b1efa665d44d91d240610a70cd0f980160e144163cf754df39dc25581b579423
  parent_name: Bard
  line_span:
    first: 2698
    count: 4
splat_clauses:
- key: source_rules
  text: '# Level 18: Superior Inspiration When you roll Initiative, you regain expended uses of Bardic
    Inspiration until you have two if you have fewer than that.'
  sha256: d18067170ec642a026bdaeb290411b03aa9a9485b91942febdc9dd88d934ccc3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 18: Superior Inspiration

<!-- SRD page 33 -->

When you roll Initiative, you regain expended uses\
of Bardic Inspiration until you have two if you have\
fewer than that.\
