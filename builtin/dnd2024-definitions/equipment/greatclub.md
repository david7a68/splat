---
splat_schema: 1
splat_id: f1b1ad07-9181-5438-a9a5-355447e5ccb0
splat_kind: item
splat_key: dnd2024:equipment/greatclub
splat_name: Greatclub
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
  properties_text: Two-Handed
  mastery: push
  weapon_category: simple melee
  price:
    value: 2
    unit: sp
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
  text: '# Greatclub Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d8", "type": "bludgeoning"
    }, "properties_text": "Two-Handed", "mastery": "push", "weapon_category": "simple melee", "price":
    { "value": 2, "unit": "sp" }, "mass": { "value": 10.0, "unit": "lb" }, "category": "weapon" }'
  sha256: 7bbfef7e5052cae6b72479772882621f9d702e900ae292d270fcde86d044cda9
  status: needs_semantic_conversion
---

# Greatclub

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d8",
    "type": "bludgeoning"
  },
  "properties_text": "Two-Handed",
  "mastery": "push",
  "weapon_category": "simple melee",
  "price": {
    "value": 2,
    "unit": "sp"
  },
  "mass": {
    "value": 10.0,
    "unit": "lb"
  },
  "category": "weapon"
}
