---
splat_schema: 1
splat_id: 5a6b1592-1922-5ae9-aadc-9501edddd8ae
splat_kind: item
splat_key: dnd2024:equipment/light_hammer
splat_name: Light Hammer
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: 1d4
    type: bludgeoning
  properties_text: Light, Thrown (Range 20/60)
  mastery: nick
  weapon_category: simple melee
  price:
    value: 2
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
  text: '# Light Hammer Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d4", "type": "bludgeoning"
    }, "properties_text": "Light, Thrown (Range 20/60)", "mastery": "nick", "weapon_category": "simple
    melee", "price": { "value": 2, "unit": "gp" }, "mass": { "value": 2.0, "unit": "lb" }, "category":
    "weapon" }'
  sha256: f81465be2332d6ba5ed64dcc6bcd247f6b8632ecf79708819762b83e7214a976
  status: needs_semantic_conversion
---

# Light Hammer

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d4",
    "type": "bludgeoning"
  },
  "properties_text": "Light, Thrown (Range 20/60)",
  "mastery": "nick",
  "weapon_category": "simple melee",
  "price": {
    "value": 2,
    "unit": "gp"
  },
  "mass": {
    "value": 2.0,
    "unit": "lb"
  },
  "category": "weapon"
}
