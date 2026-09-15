---
splat_schema: 1
splat_id: 6b0fd1dd-b906-5d1e-8c19-8ee5ac25b492
splat_kind: item
splat_key: dnd2024:equipment/javelin
splat_name: Javelin
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: 1d6
    type: piercing
  properties_text: Thrown (Range 30/120)
  mastery: slow
  weapon_category: simple melee
  price:
    value: 5
    unit: sp
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
  text: '# Javelin Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d6", "type": "piercing"
    }, "properties_text": "Thrown (Range 30/120)", "mastery": "slow", "weapon_category": "simple melee",
    "price": { "value": 5, "unit": "sp" }, "mass": { "value": 2.0, "unit": "lb" }, "category": "weapon"
    }'
  sha256: 1bac0ae6cc31c423fd15203095b1a485dc9425280c201dc7d741a5b8f6f998c6
  status: needs_semantic_conversion
---

# Javelin

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d6",
    "type": "piercing"
  },
  "properties_text": "Thrown (Range 30/120)",
  "mastery": "slow",
  "weapon_category": "simple melee",
  "price": {
    "value": 5,
    "unit": "sp"
  },
  "mass": {
    "value": 2.0,
    "unit": "lb"
  },
  "category": "weapon"
}
