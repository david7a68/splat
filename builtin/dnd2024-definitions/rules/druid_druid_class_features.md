---
splat_schema: 1
splat_id: b088cfd4-57b9-58d8-8b34-a0a817003853
splat_kind: rule
splat_key: dnd2024:rules/druid_druid_class_features
splat_name: Druid Class Features
splat_origin:
  key: dnd2024:reference/pages/041
splat_sources:
- title: SRD 5.2.1
  locator: pp. 41
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/druid_druid_class_features
  parameters:
    class:
      key: dnd2024:classes/druid
    feature_acquisition: on_reaching_feature_class_level
    level_source: class_level
    progression_source:
      key: dnd2024:tables/druid_progression
  implementation: dnd2024:rule_druid_druid_class_features
splat_contracts:
- id: dnd2024:rule_druid_druid_class_features
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
  - 41
  engine_gaps: &id001
  - dnd2024:rule_druid_druid_class_features
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: dc5ec6f236d51e08acc38a87772b898fe2d5240dbd51cb2d28e0644a69fd03bc
  parent_name: Druid
  line_span:
    first: 3396
    count: 4
splat_clauses:
- key: source_rules
  text: '# Druid Class Features As a Druid, you gain the following class features when you reach the specified
    Druid levels. These features are listed in the Druid Features table.'
  sha256: f3a94a59cd7461fa555bc3431c5c58200986794811d3d4e182e288a9bba43d88
  status: contract_specified
splat_requires_mechanics: *id001
---

# Druid Class Features

<!-- SRD page 41 -->

As a Druid, you gain the following class features\
when you reach the specified Druid levels. These\
features are listed in the Druid Features table.\
