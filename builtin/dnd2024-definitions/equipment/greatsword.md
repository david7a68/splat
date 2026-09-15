---
splat_schema: 1
splat_id: 690a9342-67db-5b7d-bc34-05af8c204771
splat_kind: item
splat_key: dnd2024:equipment/greatsword
splat_name: Greatsword
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: 2d6
    type: slashing
  properties_text: Heavy, Two-Handed
  mastery: graze
  weapon_category: martial melee
  price:
    value: 50
    unit: gp
  mass:
    value: 6.0
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
  text: '# Greatsword Source table, SRD 5.2.1 page 91. { "damage": { "expression": "2d6", "type": "slashing"
    }, "properties_text": "Heavy, Two-Handed", "mastery": "graze", "weapon_category": "martial melee",
    "price": { "value": 50, "unit": "gp" }, "mass": { "value": 6.0, "unit": "lb" }, "category": "weapon"
    }'
  sha256: b31222167cc8383c38afc2f1f2edac726be9c43346112723736ac00ca011bf75
  status: needs_semantic_conversion
---

# Greatsword

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "2d6",
    "type": "slashing"
  },
  "properties_text": "Heavy, Two-Handed",
  "mastery": "graze",
  "weapon_category": "martial melee",
  "price": {
    "value": 50,
    "unit": "gp"
  },
  "mass": {
    "value": 6.0,
    "unit": "lb"
  },
  "category": "weapon"
}
