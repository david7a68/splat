---
splat_schema: 1
splat_id: 5470b619-0b2d-50b3-a625-e084a8bc64a7
splat_kind: item
splat_key: dnd2024:equipment/hide_armor
splat_name: Hide Armor
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
    base_ac: 12
    add_dexterity_modifier: true
    dexterity_modifier_max: 2
  price:
    value: 10
    unit: gp
  mass:
    value: 12.0
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
  text: '# Hide Armor Source table, SRD 5.2.1 page 92. { "armor": { "category": "medium armor", "stealth_disadvantage":
    false, "base_ac": 12, "add_dexterity_modifier": true, "dexterity_modifier_max": 2 }, "price": { "value":
    10, "unit": "gp" }, "mass": { "value": 12.0, "unit": "lb" }, "category": "armor" }'
  sha256: 5a15d02d0ca691b492165ee2154724fe7ba01c5b6a3933a38bc53234cf051fc2
  status: needs_semantic_conversion
---

# Hide Armor

Source table, SRD 5.2.1 page 92.

{
  "armor": {
    "category": "medium armor",
    "stealth_disadvantage": false,
    "base_ac": 12,
    "add_dexterity_modifier": true,
    "dexterity_modifier_max": 2
  },
  "price": {
    "value": 10,
    "unit": "gp"
  },
  "mass": {
    "value": 12.0,
    "unit": "lb"
  },
  "category": "armor"
}
