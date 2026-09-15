---
splat_schema: 1
splat_id: 333d4f3b-01cd-5438-af55-d83cf82d06ff
splat_kind: rule
splat_key: dnd2024:rules/fighter_fighter_class_features
splat_name: Fighter Class Features
splat_origin:
  key: dnd2024:reference/pages/047
splat_sources:
- title: SRD 5.2.1
  locator: pp. 47
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/fighter_fighter_class_features
  parameters:
    class:
      key: dnd2024:classes/fighter
    feature_acquisition: on_reaching_feature_class_level
    level_source: class_level
    progression_source:
      key: dnd2024:tables/fighter_progression
  implementation: dnd2024:rule_fighter_fighter_class_features
splat_contracts:
- id: dnd2024:rule_fighter_fighter_class_features
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - character_changes
  events: []
  capabilities:
  - classes
  - advancement
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 47
  engine_gaps: &id001
  - dnd2024:rule_fighter_fighter_class_features
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 4b95c578d37cd38768ee89a6ace3affd4a6c094d02382585533b6efa2f73d0ce
  parent_name: Fighter
  line_span:
    first: 3930
    count: 4
splat_clauses:
- key: source_rules
  text: '# Fighter Class Features As a Fighter, you gain the following class features when you reach the
    specified Fighter levels. These features are listed on the Fighter Features table.'
  sha256: b68fcda1551b5b10f4c9b9152f81d481e4e33c4632d2082920f9e7e7ea645815
  status: contract_specified
splat_requires_mechanics: *id001
---

# Fighter Class Features

<!-- SRD page 47 -->

As a Fighter, you gain the following class features\
when you reach the specified Fighter levels. These\
features are listed on the Fighter Features table.\
