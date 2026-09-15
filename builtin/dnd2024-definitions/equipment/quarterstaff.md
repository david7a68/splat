---
splat_schema: 1
splat_id: 857d9b74-589d-5a41-8739-5970167b793b
splat_kind: item
splat_key: dnd2024:equipment/quarterstaff
splat_name: Quarterstaff
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: 1d6
    type: bludgeoning
  properties_text: Versatile (1d8)
  mastery: topple
  weapon_category: simple melee
  price:
    value: 2
    unit: sp
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
  text: '# Quarterstaff Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d6", "type": "bludgeoning"
    }, "properties_text": "Versatile (1d8)", "mastery": "topple", "weapon_category": "simple melee", "price":
    { "value": 2, "unit": "sp" }, "mass": { "value": 4.0, "unit": "lb" }, "category": "weapon" }'
  sha256: 76191ca500706e6017bc40550eecb71e3a5a5e24972a06b0197ceb3eccc74d4b
  status: needs_semantic_conversion
---

# Quarterstaff

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d6",
    "type": "bludgeoning"
  },
  "properties_text": "Versatile (1d8)",
  "mastery": "topple",
  "weapon_category": "simple melee",
  "price": {
    "value": 2,
    "unit": "sp"
  },
  "mass": {
    "value": 4.0,
    "unit": "lb"
  },
  "category": "weapon"
}
