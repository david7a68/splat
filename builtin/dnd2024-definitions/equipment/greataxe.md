---
splat_schema: 1
splat_id: afa7549a-80b1-5fae-b129-44c01853b6c5
splat_kind: item
splat_key: dnd2024:equipment/greataxe
splat_name: Greataxe
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: 1d12
    type: slashing
  properties_text: Heavy, Two-Handed
  mastery: cleave
  weapon_category: martial melee
  price:
    value: 30
    unit: gp
  mass:
    value: 7.0
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
  text: '# Greataxe Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d12", "type": "slashing"
    }, "properties_text": "Heavy, Two-Handed", "mastery": "cleave", "weapon_category": "martial melee",
    "price": { "value": 30, "unit": "gp" }, "mass": { "value": 7.0, "unit": "lb" }, "category": "weapon"
    }'
  sha256: d7c057a9f70bc3aecc402876651718549e6db103071fd6aeee15f3fea36ccb1b
  status: needs_semantic_conversion
---

# Greataxe

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d12",
    "type": "slashing"
  },
  "properties_text": "Heavy, Two-Handed",
  "mastery": "cleave",
  "weapon_category": "martial melee",
  "price": {
    "value": 30,
    "unit": "gp"
  },
  "mass": {
    "value": 7.0,
    "unit": "lb"
  },
  "category": "weapon"
}
