---
splat_schema: 1
splat_id: 9d3f82ca-c0c9-5643-b4e2-bb62b1882779
splat_kind: rule
splat_key: dnd2024:rules/feat_descriptions_fighting_style_feats
splat_name: Fighting Style Feats
splat_origin:
  key: dnd2024:reference/pages/087
splat_sources:
- title: SRD 5.2.1
  locator: pp. 87
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/feat_descriptions_fighting_style_feats
  parameters:
    category: fighting_style
    members:
    - key: dnd2024:feats/fighting_style_feats_archery
    - key: dnd2024:feats/fighting_style_feats_defense
    - key: dnd2024:feats/fighting_style_feats_great_weapon_fighting
    - key: dnd2024:feats/fighting_style_feats_two_weapon_fighting
  implementation: dnd2024:rules_feat_descriptions_fighting_style_feats
splat_contracts:
- id: dnd2024:rules_feat_descriptions_fighting_style_feats
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - feat_options
  events: []
  capabilities:
  - character_options
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 87
  engine_gaps: &id001
  - dnd2024:rules_feat_descriptions_fighting_style_feats
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 1c8cbabed25478c45d88f64be25e9032739db871ad6f8904ee0d30e966fb9fe1
  parent_name: Feat Descriptions
  line_span:
    first: 7543
    count: 1
splat_clauses:
- key: source_rules
  text: '# Fighting Style Feats'
  sha256: a0a4b199c1752fd4a9dc3397ba73cf406a9d50f736445376ed9ab5cc501b30fc
  status: contract_specified
splat_requires_mechanics: *id001
---

# Fighting Style Feats
