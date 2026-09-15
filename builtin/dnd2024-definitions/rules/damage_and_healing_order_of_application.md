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
  parameters: {}
splat_conversion:
  status: partial
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 17
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
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
  status: needs_semantic_conversion
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
