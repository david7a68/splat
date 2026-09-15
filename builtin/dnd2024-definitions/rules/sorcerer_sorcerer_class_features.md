---
splat_schema: 1
splat_id: 877e3432-81e8-5958-9f61-1687c16c48c9
splat_kind: rule
splat_key: dnd2024:rules/sorcerer_sorcerer_class_features
splat_name: Sorcerer Class Features
splat_origin:
  key: dnd2024:reference/pages/064
splat_sources:
- title: SRD 5.2.1
  locator: pp. 64
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/sorcerer_sorcerer_class_features
  parameters:
    class_definition:
      key: dnd2024:classes/sorcerer
    grant_features:
      level_basis: class_level
      unlock_when: class_level_gte_feature_level
      from: class_progression
    feature_unlocks:
    - class_level: 19
      feature:
        key: dnd2024:class_features/sorcerer_level_19_epic_boon
    - class_level: 1
      feature:
        key: dnd2024:class_features/sorcerer_level_1_innate_sorcery
    - class_level: 1
      feature:
        key: dnd2024:class_features/sorcerer_level_1_spellcasting
    - class_level: 20
      feature:
        key: dnd2024:class_features/sorcerer_level_20_arcane_apotheosis
    - class_level: 2
      feature:
        key: dnd2024:class_features/sorcerer_level_2_font_of_magic
    - class_level: 2
      feature:
        key: dnd2024:class_features/sorcerer_level_2_metamagic
    - class_level: 3
      feature:
        key: dnd2024:class_features/sorcerer_level_3_sorcerer_subclass
    - class_level: 4
      feature:
        key: dnd2024:class_features/sorcerer_level_4_ability_score_improvement
    - class_level: 5
      feature:
        key: dnd2024:class_features/sorcerer_level_5_sorcerous_restoration
    - class_level: 7
      feature:
        key: dnd2024:class_features/sorcerer_level_7_sorcery_incarnate
  implementation: dnd2024:rules_sorcerer_sorcerer_class_features
splat_contracts:
- id: dnd2024:rules_sorcerer_sorcerer_class_features
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
  - 64
  engine_gaps: &id001
  - dnd2024:rules_sorcerer_sorcerer_class_features
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: b8f7d9ea37303d0b28ebf2c19b5410f913d3d436d6b6b03de70520b784fc3e01
  parent_name: Sorcerer
  line_span:
    first: 5430
    count: 4
splat_clauses:
- key: source_rules
  text: '# Sorcerer Class Features As a Sorcerer, you gain the following class features when you reach
    the specified Sorcerer levels. These features are listed in the Sorcerer Features table.'
  sha256: e877c4b2654c846abce437ff8b27307cea760e11541cd9ad45e5b490f7ab1e14
  status: contract_specified
splat_requires_mechanics: *id001
---

# Sorcerer Class Features

<!-- SRD page 64 -->

As a Sorcerer, you gain the following class features\
when you reach the specified Sorcerer levels. These\
features are listed in the Sorcerer Features table.\
