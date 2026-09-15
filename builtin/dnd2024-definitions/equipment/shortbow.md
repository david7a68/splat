---
splat_schema: 1
splat_id: 0c114682-e678-5c96-857c-c90521f2f4d5
splat_kind: item
splat_key: dnd2024:equipment/shortbow
splat_name: Shortbow
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
  properties_text: Ammunition (Range 80/320; Arrow), Two-Handed
  mastery: vex
  weapon_category: simple ranged
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
  text: '# Shortbow Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d6", "type": "piercing"
    }, "properties_text": "Ammunition (Range 80/320; Arrow), Two-Handed", "mastery": "vex", "weapon_category":
    "simple ranged", "price": { "value": 25, "unit": "gp" }, "mass": { "value": 2.0, "unit": "lb" }, "category":
    "weapon" }'
  sha256: 972cffb42d110c1abc4519d7165c25a4524cb7bcb535ae6583915759a6da7854
  status: needs_semantic_conversion
---

# Shortbow

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d6",
    "type": "piercing"
  },
  "properties_text": "Ammunition (Range 80/320; Arrow), Two-Handed",
  "mastery": "vex",
  "weapon_category": "simple ranged",
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
