---
splat_schema: 1
splat_id: d9b30ace-5934-5262-8603-88580f1712ff
splat_kind: item
splat_key: dnd2024:equipment/shortsword
splat_name: Shortsword
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
  properties_text: Finesse, Light
  mastery: vex
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
  text: '# Shortsword Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d6", "type": "piercing"
    }, "properties_text": "Finesse, Light", "mastery": "vex", "weapon_category": "martial melee", "price":
    { "value": 10, "unit": "gp" }, "mass": { "value": 2.0, "unit": "lb" }, "category": "weapon" }'
  sha256: 9e98890dde8da49dfce602cec0b9d193369b45b56c2881c0159e130583992612
  status: needs_semantic_conversion
---

# Shortsword

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d6",
    "type": "piercing"
  },
  "properties_text": "Finesse, Light",
  "mastery": "vex",
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
