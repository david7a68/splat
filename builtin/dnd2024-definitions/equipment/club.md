---
splat_schema: 1
splat_id: 3c4b0347-3550-537b-ad54-8f2b43d25f9d
splat_kind: item
splat_key: dnd2024:equipment/club
splat_name: Club
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
  properties_text: Light
  mastery: slow
  weapon_category: simple melee
  price:
    value: 1
    unit: sp
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
  text: '# Club Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d4", "type": "bludgeoning"
    }, "properties_text": "Light", "mastery": "slow", "weapon_category": "simple melee", "price": { "value":
    1, "unit": "sp" }, "mass": { "value": 2.0, "unit": "lb" }, "category": "weapon" }'
  sha256: d5e6fd26ffb9d7242dfa4a3ecfa855a2a2efbe19bb213c2dc4abad471752cac8
  status: needs_semantic_conversion
---

# Club

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d4",
    "type": "bludgeoning"
  },
  "properties_text": "Light",
  "mastery": "slow",
  "weapon_category": "simple melee",
  "price": {
    "value": 1,
    "unit": "sp"
  },
  "mass": {
    "value": 2.0,
    "unit": "lb"
  },
  "category": "weapon"
}
