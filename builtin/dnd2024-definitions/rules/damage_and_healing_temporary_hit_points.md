---
splat_schema: 1
splat_id: 47399c60-6761-5182-ac5e-1a824a57b1e1
splat_kind: rule
splat_key: dnd2024:rules/damage_and_healing_temporary_hit_points
splat_name: Temporary Hit Points
splat_origin:
  key: dnd2024:reference/pages/018
splat_sources:
- title: SRD 5.2.1
  locator: pp. 18
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/damage_and_healing_temporary_hit_points
  parameters:
    apply_definitions:
    - key: dnd2024:glossary/temporary_hit_points
  implementation: dnd2024:rule_damage_and_healing_temporary_hit_points
splat_contracts:
- id: dnd2024:rule_damage_and_healing_temporary_hit_points
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
  - hit_points
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 18
  engine_gaps: &id001
  - dnd2024:rule_damage_and_healing_temporary_hit_points
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 6776ebaa64539aaa08b4cd1a8daf24ff4c35a48ddf77825e617470e2dc8689d8
  parent_name: Damage and Healing
  line_span:
    first: 1288
    count: 4
splat_clauses:
- key: source_rules
  text: '# Temporary Hit Points Some spells and other effects confer Temporary Hit Points, which are a
    buffer against losing actual Hit Points, as explained below.'
  sha256: b00de458e6e965a1e68faa399754bfab66ddd8fc5dcf17df5dcbcf744fe73bd9
  status: contract_specified
splat_requires_mechanics: *id001
---

# Temporary Hit Points

<!-- SRD page 18 -->

Some spells and other effects confer Temporary Hit\
Points, which are a buffer against losing actual Hit\
Points, as explained below.\
