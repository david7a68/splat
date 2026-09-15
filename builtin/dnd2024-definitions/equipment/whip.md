---
splat_schema: 1
splat_id: 784710b1-91c1-5b33-9394-492f4b54b64e
splat_kind: item
splat_key: dnd2024:equipment/whip
splat_name: Whip
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
  properties_text: Finesse, Reach
  mastery: slow
  weapon_category: martial melee
  price:
    value: 2
    unit: gp
  mass:
    value: 3.0
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
  text: '# Whip Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d4", "type": "slashing"
    }, "properties_text": "Finesse, Reach", "mastery": "slow", "weapon_category": "martial melee", "price":
    { "value": 2, "unit": "gp" }, "mass": { "value": 3.0, "unit": "lb" }, "category": "weapon" }'
  sha256: 14c236948746adeb1104776e6588df788bd8c6718d80a0c29f60f33ceb77f903
  status: needs_semantic_conversion
---

# Whip

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d4",
    "type": "slashing"
  },
  "properties_text": "Finesse, Reach",
  "mastery": "slow",
  "weapon_category": "martial melee",
  "price": {
    "value": 2,
    "unit": "gp"
  },
  "mass": {
    "value": 3.0,
    "unit": "lb"
  },
  "category": "weapon"
}
