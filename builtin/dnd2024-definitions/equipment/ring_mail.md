---
splat_schema: 1
splat_id: b228ce26-626e-5cf9-a96d-9834db85a2bb
splat_kind: item
splat_key: dnd2024:equipment/ring_mail
splat_name: Ring Mail
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
    base_ac: 14
    add_dexterity_modifier: false
  price:
    value: 30
    unit: gp
  mass:
    value: 40.0
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
  text: '# Ring Mail Source table, SRD 5.2.1 page 92. { "armor": { "category": "heavy armor", "stealth_disadvantage":
    true, "base_ac": 14, "add_dexterity_modifier": false }, "price": { "value": 30, "unit": "gp" }, "mass":
    { "value": 40.0, "unit": "lb" }, "category": "armor" }'
  sha256: 7a9e04a57301c5a10d147ec3880a9769ee088bd677ddb307a07dc0ebc756329f
  status: needs_semantic_conversion
---

# Ring Mail

Source table, SRD 5.2.1 page 92.

{
  "armor": {
    "category": "heavy armor",
    "stealth_disadvantage": true,
    "base_ac": 14,
    "add_dexterity_modifier": false
  },
  "price": {
    "value": 30,
    "unit": "gp"
  },
  "mass": {
    "value": 40.0,
    "unit": "lb"
  },
  "category": "armor"
}
