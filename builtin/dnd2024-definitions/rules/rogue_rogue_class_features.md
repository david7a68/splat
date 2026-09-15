---
splat_schema: 1
splat_id: 4d45fde6-670b-50b9-8649-5aecce62b64f
splat_kind: rule
splat_key: dnd2024:rules/rogue_rogue_class_features
splat_name: Rogue Class Features
splat_origin:
  key: dnd2024:reference/pages/061
splat_sources:
- title: SRD 5.2.1
  locator: pp. 61
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/rogue_rogue_class_features
  parameters:
    class:
      key: dnd2024:classes/rogue
    feature_acquisition: on_reaching_feature_class_level
    level_source: class_level
    progression_source:
      key: dnd2024:tables/rogue_progression
  implementation: dnd2024:rule_rogue_rogue_class_features
splat_contracts:
- id: dnd2024:rule_rogue_rogue_class_features
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
  - 61
  engine_gaps: &id001
  - dnd2024:rule_rogue_rogue_class_features
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 747cba4e5d134e2d4f4b21931a186b5ae5520f3e5c234d84ebe446060e82042c
  parent_name: Rogue
  line_span:
    first: 5161
    count: 4
splat_clauses:
- key: source_rules
  text: '# Rogue Class Features As a Rogue, you gain the following class features when you reach the specified
    Rogue levels. These features are listed in the Rogue Features table.'
  sha256: 48dac63d88cda48a37de6ad177e7b6cb396e4beb548c7759c1824cf955015439
  status: contract_specified
splat_requires_mechanics: *id001
---

# Rogue Class Features

<!-- SRD page 61 -->

As a Rogue, you gain the following class features\
when you reach the specified Rogue levels. These\
features are listed in the Rogue Features table.\
