---
splat_schema: 1
splat_id: 6381ca24-83c7-5194-be7a-e3af6a91d30b
splat_kind: item
splat_key: dnd2024:equipment/leather_armor
splat_name: Leather Armor
splat_origin:
  key: dnd2024:reference/pages/092
splat_sources:
- title: SRD 5.2.1
  locator: pp. 92
  url: https://www.dndbeyond.com/srd
splat_item:
  armor:
    category: light armor
    stealth_disadvantage: false
    base_ac: 11
    add_dexterity_modifier: true
  price:
    value: 10
    unit: gp
  mass:
    value: 10.0
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
  text: '# Leather Armor Source table, SRD 5.2.1 page 92. { "armor": { "category": "light armor", "stealth_disadvantage":
    false, "base_ac": 11, "add_dexterity_modifier": true }, "price": { "value": 10, "unit": "gp" }, "mass":
    { "value": 10.0, "unit": "lb" }, "category": "armor" }'
  sha256: 6eb43b5e1c555447f9e76b1087ddd7954cba14ba27e86165aed1a874c0349141
  status: needs_semantic_conversion
---

# Leather Armor

Source table, SRD 5.2.1 page 92.

{
  "armor": {
    "category": "light armor",
    "stealth_disadvantage": false,
    "base_ac": 11,
    "add_dexterity_modifier": true
  },
  "price": {
    "value": 10,
    "unit": "gp"
  },
  "mass": {
    "value": 10.0,
    "unit": "lb"
  },
  "category": "armor"
}
