---
splat_schema: 1
splat_id: 5b384c0e-7334-5e20-abc1-92df21510e3a
splat_kind: rule
splat_key: dnd2024:class_features/champion_level_7_additional_fighting_style
splat_name: 'Level 7: Additional Fighting Style'
splat_origin:
  key: dnd2024:reference/pages/049
splat_sources:
- title: SRD 5.2.1
  locator: pp. 49
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/champion_level_7_additional_fighting_style
  parameters:
    feat_choice:
      count: 1
      category: fighting_style
      additional: true
    class_scope:
      class: fighter
      minimum_level: 7
  implementation: dnd2024:class_features_champion_level_7_additional_fighting_style
splat_contracts:
- id: dnd2024:class_features_champion_level_7_additional_fighting_style
  inputs:
    actor:
      type: participant
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - feat_grants
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
  - 49
  engine_gaps: &id001
  - dnd2024:class_features_champion_level_7_additional_fighting_style
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: ce0acd15495cdafbb474c0562dd9153861b00a2e58c47f2f66cda2196b754574
  parent_name: Champion
  line_span:
    first: 4067
    count: 2
splat_clauses:
- key: source_rules
  text: '# Level 7: Additional Fighting Style You gain another Fighting Style feat of your choice.'
  sha256: 8d5bf3d49d75800633d269f38830377136b471468cad7cb90a4d005f9abea65b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 7: Additional Fighting Style

<!-- SRD page 49 -->

You gain another Fighting Style feat of your choice.\
