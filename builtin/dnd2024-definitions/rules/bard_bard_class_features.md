---
splat_schema: 1
splat_id: 87aea1b6-3224-5ccb-8dba-46af7a6ca1ae
splat_kind: rule
splat_key: dnd2024:rules/bard_bard_class_features
splat_name: Bard Class Features
splat_origin:
  key: dnd2024:reference/pages/031
splat_sources:
- title: SRD 5.2.1
  locator: pp. 31
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/bard_bard_class_features
  parameters:
    class_definition:
      key: dnd2024:classes/bard
    grant_features:
      level_basis: class_level
      unlock_when: class_level_gte_feature_level
      from: class_progression
    feature_unlocks:
    - class_level: 10
      feature:
        key: dnd2024:class_features/bard_level_10_magical_secrets
    - class_level: 18
      feature:
        key: dnd2024:class_features/bard_level_18_superior_inspiration
    - class_level: 19
      feature:
        key: dnd2024:class_features/bard_level_19_epic_boon
    - class_level: 1
      feature:
        key: dnd2024:class_features/bard_level_1_bardic_inspiration
    - class_level: 1
      feature:
        key: dnd2024:class_features/bard_level_1_spellcasting
    - class_level: 20
      feature:
        key: dnd2024:class_features/bard_level_20_words_of_creation
    - class_level: 2
      feature:
        key: dnd2024:class_features/bard_level_2_expertise
    - class_level: 2
      feature:
        key: dnd2024:class_features/bard_level_2_jack_of_all_trades
    - class_level: 3
      feature:
        key: dnd2024:class_features/bard_level_3_bard_subclass
    - class_level: 4
      feature:
        key: dnd2024:class_features/bard_level_4_ability_score_improvement
    - class_level: 5
      feature:
        key: dnd2024:class_features/bard_level_5_font_of_inspiration
    - class_level: 7
      feature:
        key: dnd2024:class_features/bard_level_7_countercharm
  implementation: dnd2024:rules_bard_bard_class_features
splat_contracts:
- id: dnd2024:rules_bard_bard_class_features
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
  - 31
  engine_gaps: &id001
  - dnd2024:rules_bard_bard_class_features
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 40698209502d5aabdb4e89c399663b49d59ba5cb0dd6ab8770cb206a7d76472e
  parent_name: Bard
  line_span:
    first: 2549
    count: 4
splat_clauses:
- key: source_rules
  text: '# Bard Class Features As a Bard, you gain the following class features when you reach the specified
    Bard levels. These features are listed in the Bard Features table.'
  sha256: 2aac1e02dfae4fb09e71bb52aa8ecb929685ccc9ab5bb03c9bc8c163a0575820
  status: contract_specified
splat_requires_mechanics: *id001
---

# Bard Class Features

<!-- SRD page 31 -->

As a Bard, you gain the following class features\
when you reach the specified Bard levels. These fea-\
tures are listed in the Bard Features table.\
