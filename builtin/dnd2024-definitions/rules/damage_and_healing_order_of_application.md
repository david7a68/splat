---
splat_schema: 1
splat_id: 7ee21a0e-cede-5627-82e9-5c97418aff3d
splat_kind: rule
splat_key: dnd2024:rules/damage_and_healing_order_of_application
splat_name: Order of Application
splat_origin:
  key: dnd2024:reference/pages/017
splat_sources:
- title: SRD 5.2.1
  locator: pp. 17
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/damage_and_healing_order_of_application
  parameters:
    ordered_operations:
    - bonuses_penalties_multipliers
    - resistance
    - vulnerability
    resistance:
      divide: 2
      rounding: floor
    vulnerability:
      multiply: 2
    example:
      initial: 28
      flat_adjustment: -5
      after_adjustment: 23
      after_resistance: 11
      after_vulnerability: 22
  implementation: dnd2024:rule_damage_and_healing_order_of_application
splat_contracts:
- id: dnd2024:rule_damage_and_healing_order_of_application
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - damage
  events: []
  capabilities:
  - damage
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 17
  engine_gaps: &id001
  - dnd2024:rule_damage_and_healing_order_of_application
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 1f734eae98724b59dbbb84f34954c7b28a0e0c36a41cadcba79c6670de3211ec
  parent_name: Damage and Healing
  line_span:
    first: 1186
    count: 12
splat_clauses:
- key: source_rules
  text: '# Order of Application Modifiers to damage are applied in the following order: adjustments such
    as bonuses, penalties, or multipliers are applied first; Resistance is applied second; and Vulnerability
    is applied third. For example, a creature has Resistance to all damage and Vulnerability to Fire damage,
    and it’s within a magical aura that reduces all damage by 5. If it takes 28 Fire damage, the damage
    is first reduced by 5 (to 23), then halved for the creature’s Resistance (and rounded down to 11),
    then doubled for its Vulnerability (to 22).'
  sha256: 9ed06b8410e9322a8bb19a3faebdba78264ef21a6fa87b4a8a003691418c2031
  status: contract_specified
splat_requires_mechanics: *id001
---

# Order of Application

<!-- SRD page 17 -->

Modifiers to damage are applied in the following\
order: adjustments such as bonuses, penalties, or\
multipliers are applied first; Resistance is applied\
second; and Vulnerability is applied third.\
For example, a creature has Resistance to all dam-\
age and Vulnerability to Fire damage, and it’s within\
a magical aura that reduces all damage by 5. If it\
takes 28 Fire damage, the damage is first reduced\
by 5 (to 23), then halved for the creature’s Resis-\
tance (and rounded down to 11), then doubled for\
its Vulnerability (to 22).\
