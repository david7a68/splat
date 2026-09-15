---
splat_schema: 1
splat_id: 37959a76-5c7b-5ef8-9952-5be893e32594
splat_kind: item
splat_key: dnd2024:equipment/warhammer
splat_name: Warhammer
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
  properties_text: Versatile (1d10)
  mastery: push
  weapon_category: martial melee
  price:
    value: 15
    unit: gp
  mass:
    value: 5.0
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
  text: '# Warhammer Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d8", "type": "bludgeoning"
    }, "properties_text": "Versatile (1d10)", "mastery": "push", "weapon_category": "martial melee", "price":
    { "value": 15, "unit": "gp" }, "mass": { "value": 5.0, "unit": "lb" }, "category": "weapon" }'
  sha256: d53da4fa59a978d719ed389b0369d240efc2a79cdc2c3257d66ee21e33f04c73
  status: needs_semantic_conversion
---

# Warhammer

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d8",
    "type": "bludgeoning"
  },
  "properties_text": "Versatile (1d10)",
  "mastery": "push",
  "weapon_category": "martial melee",
  "price": {
    "value": 15,
    "unit": "gp"
  },
  "mass": {
    "value": 5.0,
    "unit": "lb"
  },
  "category": "weapon"
}
