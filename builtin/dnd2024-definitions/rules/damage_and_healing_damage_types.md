---
splat_schema: 1
splat_id: 7381fa3f-5a33-57a1-8315-b62b2f9033c6
splat_kind: rule
splat_key: dnd2024:rules/damage_and_healing_damage_types
splat_name: Damage Types
splat_origin:
  key: dnd2024:reference/pages/016
splat_sources:
- title: SRD 5.2.1
  locator: pp. 16, 17
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/damage_and_healing_damage_types
  parameters:
    apply_definitions:
    - key: dnd2024:glossary/damage_types
    each_damage_instance_has_type: true
    inherent_type_rules: false
  implementation: dnd2024:rule_damage_and_healing_damage_types
splat_contracts:
- id: dnd2024:rule_damage_and_healing_damage_types
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - resolution
  events: []
  capabilities:
  - damage
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 16
  - 17
  engine_gaps: &id001
  - dnd2024:rule_damage_and_healing_damage_types
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 6a139914e39a394d7c9f307303878d97ea1f5ad9fec6b8a1943a0618f4b469fd
  parent_name: Damage and Healing
  line_span:
    first: 1165
    count: 5
splat_clauses:
- key: source_rules
  text: '# Damage Types Each instance of damage has a type, like Fire or Slashing. Damage types are listed
    in “Rules Glossary” and have no rules of their own, but other rules, such as Resistance, rely on damage
    types.'
  sha256: 37cd477253fe83939633a3a42708ecbf97f97980a84380d250197a0417bb448e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Damage Types

<!-- SRD page 16 -->

Each instance of damage has a type, like Fire\
or Slashing. Damage types are listed in “Rules\

<!-- SRD page 17 -->

Glossary” and have no rules of their own, but other\
rules, such as Resistance, rely on damage types.\
