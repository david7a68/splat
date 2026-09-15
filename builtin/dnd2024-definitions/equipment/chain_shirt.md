---
splat_schema: 1
splat_id: c540d98d-1960-508d-997f-581ab48f8d57
splat_kind: item
splat_key: dnd2024:equipment/chain_shirt
splat_name: Chain Shirt
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
    base_ac: 13
    add_dexterity_modifier: true
    dexterity_modifier_max: 2
  price:
    value: 50
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
  text: '# Chain Shirt Source table, SRD 5.2.1 page 92. { "armor": { "category": "medium armor", "stealth_disadvantage":
    false, "base_ac": 13, "add_dexterity_modifier": true, "dexterity_modifier_max": 2 }, "price": { "value":
    50, "unit": "gp" }, "mass": { "value": 20.0, "unit": "lb" }, "category": "armor" }'
  sha256: e370f1b43d725825f9c6b719b99fcf7dc5b02d48a4d0055b29880a0def72e074
  status: needs_semantic_conversion
---

# Chain Shirt

Source table, SRD 5.2.1 page 92.

{
  "armor": {
    "category": "medium armor",
    "stealth_disadvantage": false,
    "base_ac": 13,
    "add_dexterity_modifier": true,
    "dexterity_modifier_max": 2
  },
  "price": {
    "value": 50,
    "unit": "gp"
  },
  "mass": {
    "value": 20.0,
    "unit": "lb"
  },
  "category": "armor"
}
