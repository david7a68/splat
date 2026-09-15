---
splat_schema: 1
splat_id: bb982fc6-7470-5bfe-aab7-24ea22754a08
splat_kind: rule
splat_key: dnd2024:class_features/warlock_level_19_epic_boon
splat_name: 'Level 19: Epic Boon'
splat_origin:
  key: dnd2024:reference/pages/072
splat_sources:
- title: SRD 5.2.1
  locator: pp. 72
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/warlock_level_19_epic_boon
  parameters:
    feat_choice:
      count: 1
      category_preference: epic_boon
      allowed_categories: any
      must_meet_prerequisites: true
    class_scope:
      class: warlock
      minimum_level: 19
  implementation: dnd2024:class_features_warlock_level_19_epic_boon
splat_contracts:
- id: dnd2024:class_features_warlock_level_19_epic_boon
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
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 72
  engine_gaps: &id001
  - dnd2024:class_features_warlock_level_19_epic_boon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 6c0b05b4814ea9803cd904a59c474945555bda37cb0135118e38f167d215a2b6
  parent_name: Warlock
  line_span:
    first: 6134
    count: 4
splat_clauses:
- key: source_rules
  text: '# Level 19: Epic Boon You gain an Epic Boon feat (see “Feats”) or another feat of your choice
    for which you qualify. Boon of Fate is recommended.'
  sha256: 98f4d20838b7a7fd33e0049c2879526f257319d9db5e8a845f59c21827314a8c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 19: Epic Boon

<!-- SRD page 72 -->

You gain an Epic Boon feat (see “Feats”) or another\
feat of your choice for which you qualify. Boon of\
Fate is recommended.\
