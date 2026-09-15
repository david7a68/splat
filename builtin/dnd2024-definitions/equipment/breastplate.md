---
splat_schema: 1
splat_id: 66cac452-6d42-5910-8f7b-c31e6aea882b
splat_kind: item
splat_key: dnd2024:equipment/breastplate
splat_name: Breastplate
splat_origin:
  key: dnd2024:reference/pages/092
splat_sources:
- title: SRD 5.2.1
  locator: pp. 92
  url: https://www.dndbeyond.com/srd
splat_item:
  armor:
    category: medium armor
    stealth_disadvantage: false
    base_ac: 14
    add_dexterity_modifier: true
    dexterity_modifier_max: 2
  price:
    value: 400
    unit: gp
  mass:
    value: 20.0
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
  text: '# Breastplate Source table, SRD 5.2.1 page 92. { "armor": { "category": "medium armor", "stealth_disadvantage":
    false, "base_ac": 14, "add_dexterity_modifier": true, "dexterity_modifier_max": 2 }, "price": { "value":
    400, "unit": "gp" }, "mass": { "value": 20.0, "unit": "lb" }, "category": "armor" }'
  sha256: c3cb7b260e092a8e1114722ecf1c9dc3daeee0e539dbe702924dc2fd3260f168
  status: needs_semantic_conversion
---

# Breastplate

Source table, SRD 5.2.1 page 92.

{
  "armor": {
    "category": "medium armor",
    "stealth_disadvantage": false,
    "base_ac": 14,
    "add_dexterity_modifier": true,
    "dexterity_modifier_max": 2
  },
  "price": {
    "value": 400,
    "unit": "gp"
  },
  "mass": {
    "value": 20.0,
    "unit": "lb"
  },
  "category": "armor"
}
