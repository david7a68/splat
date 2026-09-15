---
splat_schema: 1
splat_id: c355d0e9-a581-5620-8a4c-258b0365948c
splat_kind: rule
splat_key: dnd2024:feats/fighting_style_feats_archery
splat_name: Archery
splat_origin:
  key: dnd2024:reference/pages/087
splat_sources:
- title: SRD 5.2.1
  locator: pp. 87
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:feats/fighting_style_feats_archery
  parameters:
    feat_category: fighting_style
    prerequisites:
      feature: fighting_style
    attack_bonus:
      weapon_category: ranged
      amount: 2
  implementation: dnd2024:feats_fighting_style_feats_archery
splat_contracts:
- id: dnd2024:feats_fighting_style_feats_archery
  inputs:
    actor:
      type: participant
      required: true
    event:
      required: false
      type: event
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - attack.roll
  capabilities:
  - character_features
splat_conversion:
  status: specified
  category: feat
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 87
  engine_gaps: &id001
  - dnd2024:feats_fighting_style_feats_archery
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 68ca5b5e8b242023719423e130afc98b8be27b8d8f01910de8febd7f84f51fb8
  parent_name: Fighting Style Feats
  line_span:
    first: 7544
    count: 5
splat_clauses:
- key: source_rules
  text: '# Archery Fighting Style Feat (Prerequisite: Fighting Style Feature) You gain a +2 bonus to attack
    rolls you make with Ranged weapons.'
  sha256: 4b6a5df4cf37dec7e33dcc39923cd7a8712a5faaae0168fe72dfd1dbf287f1d9
  status: contract_specified
splat_requires_mechanics: *id001
---

# Archery

<!-- SRD page 87 -->

Fighting Style Feat (Prerequisite: Fighting Style\
Feature)\
You gain a +2 bonus to attack rolls you make with\
Ranged weapons.\
