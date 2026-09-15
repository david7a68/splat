---
splat_schema: 1
splat_id: 4ad21685-ad9c-541d-aaa1-4272f5c22121
splat_kind: item
splat_key: dnd2024:equipment/morningstar
splat_name: Morningstar
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
  properties_text: —
  mastery: sap
  weapon_category: martial melee
  price:
    value: 15
    unit: gp
  mass:
    value: 4.0
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
  text: '# Morningstar Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d8", "type": "piercing"
    }, "properties_text": "—", "mastery": "sap", "weapon_category": "martial melee", "price": { "value":
    15, "unit": "gp" }, "mass": { "value": 4.0, "unit": "lb" }, "category": "weapon" }'
  sha256: 8cc14139103dfb4ffec5a57280227670b428b76a989d424a26103751484be81e
  status: needs_semantic_conversion
---

# Morningstar

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d8",
    "type": "piercing"
  },
  "properties_text": "—",
  "mastery": "sap",
  "weapon_category": "martial melee",
  "price": {
    "value": 15,
    "unit": "gp"
  },
  "mass": {
    "value": 4.0,
    "unit": "lb"
  },
  "category": "weapon"
}
