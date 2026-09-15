---
splat_schema: 1
splat_id: a8237060-0ccc-59a5-8e75-f13f410956f1
splat_kind: rule
splat_key: dnd2024:rules/creating_a_background_2_choose_a_feat
splat_name: '2: Choose a Feat'
splat_origin:
  key: dnd2024:reference/pages/193
splat_sources:
- title: SRD 5.2.1
  locator: pp. 193
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/creating_a_background_2_choose_a_feat
  parameters:
    feat_choice:
      count: 1
      category: origin
  implementation: dnd2024:rules_creating_a_background_2_choose_a_feat
splat_contracts:
- id: dnd2024:rules_creating_a_background_2_choose_a_feat
  inputs:
    actor:
      type: participant
      required: true
    choice:
      type: choice
      required: true
  outputs:
  - character_updates
  events: []
  capabilities:
  - character_creation
  - progression
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 193
  engine_gaps: &id001
  - dnd2024:rules_creating_a_background_2_choose_a_feat
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: bc5b8f55b30a839847ea90bc40ff5f16105134997555b9dd2b96da2f7600b259
  parent_name: Creating a Background
  line_span:
    first: 9416
    count: 3
splat_clauses:
- key: source_rules
  text: '# 2: Choose a Feat Choose one feat from the Origin category. See “Origin Feats” for examples
    of Origin feats.'
  sha256: f578635ebf37d3ca441d14bdc5dd28be4cd59131a94e4a135d1e3175afb44694
  status: contract_specified
splat_requires_mechanics: *id001
---

# 2: Choose a Feat

<!-- SRD page 193 -->

Choose one feat from the Origin category. See “Ori-\
gin Feats” for examples of Origin feats.\
