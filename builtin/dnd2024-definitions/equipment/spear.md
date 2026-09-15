---
splat_schema: 1
splat_id: 6e3e7154-a70b-5ce0-9a88-aa2049957177
splat_kind: item
splat_key: dnd2024:equipment/spear
splat_name: Spear
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
  properties_text: Thrown (Range 20/60), Versatile (1d8)
  mastery: sap
  weapon_category: simple melee
  price:
    value: 1
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
  text: '# Spear Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d6", "type": "piercing"
    }, "properties_text": "Thrown (Range 20/60), Versatile (1d8)", "mastery": "sap", "weapon_category":
    "simple melee", "price": { "value": 1, "unit": "gp" }, "mass": { "value": 3.0, "unit": "lb" }, "category":
    "weapon" }'
  sha256: 70457594c25c74066a503f2caf419e08c2852fc88a42181e99b795d8298c1dab
  status: needs_semantic_conversion
---

# Spear

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d6",
    "type": "piercing"
  },
  "properties_text": "Thrown (Range 20/60), Versatile (1d8)",
  "mastery": "sap",
  "weapon_category": "simple melee",
  "price": {
    "value": 1,
    "unit": "gp"
  },
  "mass": {
    "value": 3.0,
    "unit": "lb"
  },
  "category": "weapon"
}
