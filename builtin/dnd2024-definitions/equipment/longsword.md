---
splat_schema: 1
splat_id: 48f20442-0972-53fc-8c26-1739e83a236a
splat_kind: item
splat_key: dnd2024:equipment/longsword
splat_name: Longsword
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: 1d8
    type: slashing
  properties_text: Versatile (1d10)
  mastery: sap
  weapon_category: martial melee
  price:
    value: 15
    unit: gp
  mass:
    value: 3.0
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
  text: '# Longsword Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d8", "type": "slashing"
    }, "properties_text": "Versatile (1d10)", "mastery": "sap", "weapon_category": "martial melee", "price":
    { "value": 15, "unit": "gp" }, "mass": { "value": 3.0, "unit": "lb" }, "category": "weapon" }'
  sha256: 64d7961b8c51f8902f8d77c0503b00c7013821833061f8a5d95242a0b0cb9b43
  status: needs_semantic_conversion
---

# Longsword

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d8",
    "type": "slashing"
  },
  "properties_text": "Versatile (1d10)",
  "mastery": "sap",
  "weapon_category": "martial melee",
  "price": {
    "value": 15,
    "unit": "gp"
  },
  "mass": {
    "value": 3.0,
    "unit": "lb"
  },
  "category": "weapon"
}
