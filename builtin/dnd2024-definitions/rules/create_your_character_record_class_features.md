---
splat_schema: 1
splat_id: 83a794df-efa7-5a56-97c1-fd942816bf68
splat_kind: rule
splat_key: dnd2024:rules/create_your_character_record_class_features
splat_name: Record Class Features
splat_origin:
  key: dnd2024:reference/pages/022
splat_sources:
- title: SRD 5.2.1
  locator: pp. 22
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/create_your_character_record_class_features
  parameters:
    class_feature_level: 1
    resolve_all_feature_choices: true
    features_source: selected_class_feature_table
  implementation: dnd2024:rules_create_your_character_record_class_features
splat_contracts:
- id: dnd2024:rules_create_your_character_record_class_features
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - class_features
  - feature_choices
  events: []
  capabilities:
  - character_creation
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 22
  engine_gaps: &id001
  - dnd2024:rules_create_your_character_record_class_features
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 844ba2c75a36b4434333776350e4c327f3a8d91b1d82d799bf180f8fea2382e4
  parent_name: Create Your Character
  line_span:
    first: 1639
    count: 6
splat_clauses:
- key: source_rules
  text: '# Record Class Features Look at your class’s feature table in “Classes,” and write down the level
    1 features. The class features are detailed there too. Some class features offer choices. Make sure
    to read all your features and make any offered choices.'
  sha256: 3207a05b5162cd69abee9567677082852dc47a860ab66c405a1f9bc65c611cb2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Record Class Features

<!-- SRD page 22 -->

Look at your class’s feature table in “Classes,” and\
write down the level 1 features. The class features\
are detailed there too.\
Some class features offer choices. Make sure to\
read all your features and make any offered choices.\
