---
splat_schema: 1
splat_id: 0760be8d-6832-5ad3-908d-6896e6a8e16f
splat_kind: item
splat_key: dnd2024:equipment/rapier
splat_name: Rapier
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: 1d8
    type: piercing
  properties_text: Finesse
  mastery: vex
  weapon_category: martial melee
  price:
    value: 25
    unit: gp
  mass:
    value: 2.0
    unit: lb
  category: weapon
splat_conversion:
  status: partial
  category: equipment
  source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
  source_path: builtin/srd-5.2.1/source.txt
  pages:
  - 91
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Rapier Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d8", "type": "piercing"
    }, "properties_text": "Finesse", "mastery": "vex", "weapon_category": "martial melee", "price": {
    "value": 25, "unit": "gp" }, "mass": { "value": 2.0, "unit": "lb" }, "category": "weapon" }'
  sha256: 13cc156b327e619aea52e21764e2572677b93e90aa6d5fe7dde51d822a3637d6
  status: needs_semantic_conversion
---

# Rapier

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d8",
    "type": "piercing"
  },
  "properties_text": "Finesse",
  "mastery": "vex",
  "weapon_category": "martial melee",
  "price": {
    "value": 25,
    "unit": "gp"
  },
  "mass": {
    "value": 2.0,
    "unit": "lb"
  },
  "category": "weapon"
}
