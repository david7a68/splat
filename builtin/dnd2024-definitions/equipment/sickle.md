---
splat_schema: 1
splat_id: 0a69bbd7-90e7-52ff-8275-9d8cbc5959be
splat_kind: item
splat_key: dnd2024:equipment/sickle
splat_name: Sickle
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: 1d4
    type: slashing
  properties_text: Light
  mastery: nick
  weapon_category: simple melee
  price:
    value: 1
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
  text: '# Sickle Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d4", "type": "slashing"
    }, "properties_text": "Light", "mastery": "nick", "weapon_category": "simple melee", "price": { "value":
    1, "unit": "gp" }, "mass": { "value": 2.0, "unit": "lb" }, "category": "weapon" }'
  sha256: 96021af32cb7d7575508b069b5a8ef27d579e5682879da7e4baa7b8c4eae483f
  status: needs_semantic_conversion
---

# Sickle

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d4",
    "type": "slashing"
  },
  "properties_text": "Light",
  "mastery": "nick",
  "weapon_category": "simple melee",
  "price": {
    "value": 1,
    "unit": "gp"
  },
  "mass": {
    "value": 2.0,
    "unit": "lb"
  },
  "category": "weapon"
}
