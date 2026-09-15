---
splat_schema: 1
splat_id: 121b2a25-1d9a-5f2b-bb67-8db7c489c366
splat_kind: item
splat_key: dnd2024:equipment/splint_armor
splat_name: Splint Armor
splat_origin:
  key: dnd2024:reference/pages/092
splat_sources:
- title: SRD 5.2.1
  locator: pp. 92
  url: https://www.dndbeyond.com/srd
splat_item:
  armor:
    category: heavy armor
    stealth_disadvantage: true
    base_ac: 17
    add_dexterity_modifier: false
    strength_minimum: 15
  price:
    value: 200
    unit: gp
  mass:
    value: 60.0
    unit: lb
  category: armor
splat_conversion:
  status: partial
  category: equipment
  source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
  source_path: builtin/srd-5.2.1/source.txt
  pages:
  - 92
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Splint Armor Source table, SRD 5.2.1 page 92. { "armor": { "category": "heavy armor", "stealth_disadvantage":
    true, "base_ac": 17, "add_dexterity_modifier": false, "strength_minimum": 15 }, "price": { "value":
    200, "unit": "gp" }, "mass": { "value": 60.0, "unit": "lb" }, "category": "armor" }'
  sha256: f6e82d2414e3b4021b0580a0cd562d27503f0460b509d3e3c02f044457ba889c
  status: needs_semantic_conversion
---

# Splint Armor

Source table, SRD 5.2.1 page 92.

{
  "armor": {
    "category": "heavy armor",
    "stealth_disadvantage": true,
    "base_ac": 17,
    "add_dexterity_modifier": false,
    "strength_minimum": 15
  },
  "price": {
    "value": 200,
    "unit": "gp"
  },
  "mass": {
    "value": 60.0,
    "unit": "lb"
  },
  "category": "armor"
}
