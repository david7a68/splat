---
splat_schema: 1
splat_id: dc25b6ca-51bc-5670-9d90-7e3ee3861b9a
splat_kind: item
splat_key: dnd2024:equipment/chain_mail
splat_name: Chain Mail
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
    base_ac: 16
    add_dexterity_modifier: false
    strength_minimum: 13
  price:
    value: 75
    unit: gp
  mass:
    value: 55.0
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
  text: '# Chain Mail Source table, SRD 5.2.1 page 92. { "armor": { "category": "heavy armor", "stealth_disadvantage":
    true, "base_ac": 16, "add_dexterity_modifier": false, "strength_minimum": 13 }, "price": { "value":
    75, "unit": "gp" }, "mass": { "value": 55.0, "unit": "lb" }, "category": "armor" }'
  sha256: ca28eb5a671748fe5701b2fd9ffaf9799c7087c226cbca4c714be804f85808c6
  status: needs_semantic_conversion
---

# Chain Mail

Source table, SRD 5.2.1 page 92.

{
  "armor": {
    "category": "heavy armor",
    "stealth_disadvantage": true,
    "base_ac": 16,
    "add_dexterity_modifier": false,
    "strength_minimum": 13
  },
  "price": {
    "value": 75,
    "unit": "gp"
  },
  "mass": {
    "value": 55.0,
    "unit": "lb"
  },
  "category": "armor"
}
