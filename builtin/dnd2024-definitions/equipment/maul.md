---
splat_schema: 1
splat_id: 91c6c731-ec05-5094-91d8-e2ecc313ae6a
splat_kind: item
splat_key: dnd2024:equipment/maul
splat_name: Maul
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: 2d6
    type: bludgeoning
  properties_text: Heavy, Two-Handed
  mastery: topple
  weapon_category: martial melee
  price:
    value: 10
    unit: gp
  mass:
    value: 10.0
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
  text: '# Maul Source table, SRD 5.2.1 page 91. { "damage": { "expression": "2d6", "type": "bludgeoning"
    }, "properties_text": "Heavy, Two-Handed", "mastery": "topple", "weapon_category": "martial melee",
    "price": { "value": 10, "unit": "gp" }, "mass": { "value": 10.0, "unit": "lb" }, "category": "weapon"
    }'
  sha256: 6fda1590809f6d473c050ac98b58d934c3eb5d1e0638a80392e20d70b5bec58c
  status: needs_semantic_conversion
---

# Maul

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "2d6",
    "type": "bludgeoning"
  },
  "properties_text": "Heavy, Two-Handed",
  "mastery": "topple",
  "weapon_category": "martial melee",
  "price": {
    "value": 10,
    "unit": "gp"
  },
  "mass": {
    "value": 10.0,
    "unit": "lb"
  },
  "category": "weapon"
}
