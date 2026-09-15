---
splat_schema: 1
splat_id: 864fa3a5-e3e6-5913-96e1-16e90c998849
splat_kind: rule
splat_key: dnd2024:rules/wizard_wizard_class_features
splat_name: Wizard Class Features
splat_origin:
  key: dnd2024:reference/pages/077
splat_sources:
- title: SRD 5.2.1
  locator: pp. 77
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/wizard_wizard_class_features
  parameters:
    class:
      key: dnd2024:classes/wizard
    feature_acquisition: on_reaching_feature_class_level
    level_source: class_level
    progression_source:
      key: dnd2024:tables/wizard_progression
  implementation: dnd2024:rule_wizard_wizard_class_features
splat_contracts:
- id: dnd2024:rule_wizard_wizard_class_features
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
  - 77
  engine_gaps: &id001
  - dnd2024:rule_wizard_wizard_class_features
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 223f3a6b09dd2ea5ed2ecd7e709c791839ca156564d8819ef4c0ba1162362834
  parent_name: Wizard
  line_span:
    first: 6580
    count: 4
splat_clauses:
- key: source_rules
  text: '# Wizard Class Features As a Wizard, you gain the following class features when you reach the
    specified Wizard levels. These features are listed in the Wizard Features table.'
  sha256: e713b6da333882362af54e57df1f16858461bf03c1d2efd23bd56275c6571bf0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Wizard Class Features

<!-- SRD page 77 -->

As a Wizard, you gain the following class features\
when you reach the specified Wizard levels. These\
features are listed in the Wizard Features table.\
