---
splat_schema: 1
splat_id: e6b67cd5-4d3e-50b6-b069-1beb2d886330
splat_kind: item
splat_key: dnd2024:equipment/battleaxe
splat_name: Battleaxe
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
  mastery: topple
  weapon_category: martial melee
  price:
    value: 10
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
  text: '# Battleaxe Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d8", "type": "slashing"
    }, "properties_text": "Versatile (1d10)", "mastery": "topple", "weapon_category": "martial melee",
    "price": { "value": 10, "unit": "gp" }, "mass": { "value": 4.0, "unit": "lb" }, "category": "weapon"
    }'
  sha256: d2f5bb63f32e1bcc327288a14f89fc7059f0eda9ecf676fbc1274c548f9e02b5
  status: needs_semantic_conversion
---

# Battleaxe

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d8",
    "type": "slashing"
  },
  "properties_text": "Versatile (1d10)",
  "mastery": "topple",
  "weapon_category": "martial melee",
  "price": {
    "value": 10,
    "unit": "gp"
  },
  "mass": {
    "value": 4.0,
    "unit": "lb"
  },
  "category": "weapon"
}
