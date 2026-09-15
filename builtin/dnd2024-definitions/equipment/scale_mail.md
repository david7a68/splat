---
splat_schema: 1
splat_id: 2138f9fc-5f7f-519d-a700-953e8a676046
splat_kind: item
splat_key: dnd2024:equipment/scale_mail
splat_name: Scale Mail
splat_origin:
  key: dnd2024:reference/pages/092
splat_sources:
- title: SRD 5.2.1
  locator: pp. 92
  url: https://www.dndbeyond.com/srd
splat_item:
  armor:
    category: medium armor
    stealth_disadvantage: true
    base_ac: 14
    add_dexterity_modifier: true
    dexterity_modifier_max: 2
  price:
    value: 50
    unit: gp
  mass:
    value: 45.0
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
  text: '# Scale Mail Source table, SRD 5.2.1 page 92. { "armor": { "category": "medium armor", "stealth_disadvantage":
    true, "base_ac": 14, "add_dexterity_modifier": true, "dexterity_modifier_max": 2 }, "price": { "value":
    50, "unit": "gp" }, "mass": { "value": 45.0, "unit": "lb" }, "category": "armor" }'
  sha256: 0bfcde1e7975efdd69564c0ffdbeac867d31f1bcc2626839a66c208e0f70089c
  status: needs_semantic_conversion
---

# Scale Mail

Source table, SRD 5.2.1 page 92.

{
  "armor": {
    "category": "medium armor",
    "stealth_disadvantage": true,
    "base_ac": 14,
    "add_dexterity_modifier": true,
    "dexterity_modifier_max": 2
  },
  "price": {
    "value": 50,
    "unit": "gp"
  },
  "mass": {
    "value": 45.0,
    "unit": "lb"
  },
  "category": "armor"
}
