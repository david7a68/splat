---
splat_schema: 1
splat_id: 5014d8a6-e3cc-50b0-986e-e8454591d066
splat_kind: item
splat_key: dnd2024:equipment/pike
splat_name: Pike
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: 1d10
    type: piercing
  properties_text: Heavy, Reach, Two-Handed
  mastery: push
  weapon_category: martial melee
  price:
    value: 5
    unit: gp
  mass:
    value: 18.0
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
  text: '# Pike Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d10", "type": "piercing"
    }, "properties_text": "Heavy, Reach, Two-Handed", "mastery": "push", "weapon_category": "martial melee",
    "price": { "value": 5, "unit": "gp" }, "mass": { "value": 18.0, "unit": "lb" }, "category": "weapon"
    }'
  sha256: cda144b4f10a870c0b82332419fe59a2acd5d347ceb1e4c361a9b7bc05f6e8f8
  status: needs_semantic_conversion
---

# Pike

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d10",
    "type": "piercing"
  },
  "properties_text": "Heavy, Reach, Two-Handed",
  "mastery": "push",
  "weapon_category": "martial melee",
  "price": {
    "value": 5,
    "unit": "gp"
  },
  "mass": {
    "value": 18.0,
    "unit": "lb"
  },
  "category": "weapon"
}
