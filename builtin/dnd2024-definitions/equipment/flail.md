---
splat_schema: 1
splat_id: d5b5cbee-22de-545c-b755-c4b91dc3f33e
splat_kind: item
splat_key: dnd2024:equipment/flail
splat_name: Flail
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: 1d8
    type: bludgeoning
  properties_text: —
  mastery: sap
  weapon_category: martial melee
  price:
    value: 10
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
  text: '# Flail Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d8", "type": "bludgeoning"
    }, "properties_text": "—", "mastery": "sap", "weapon_category": "martial melee", "price": { "value":
    10, "unit": "gp" }, "mass": { "value": 2.0, "unit": "lb" }, "category": "weapon" }'
  sha256: 3c621adc103cad3bb91e84208b5222cc921f4b6c5e1f301f45a7e9f4482b7a20
  status: needs_semantic_conversion
---

# Flail

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d8",
    "type": "bludgeoning"
  },
  "properties_text": "—",
  "mastery": "sap",
  "weapon_category": "martial melee",
  "price": {
    "value": 10,
    "unit": "gp"
  },
  "mass": {
    "value": 2.0,
    "unit": "lb"
  },
  "category": "weapon"
}
