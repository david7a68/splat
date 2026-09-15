---
splat_schema: 1
splat_id: 889b763b-f4d3-54b4-9b29-a2e2af1dfa0a
splat_kind: item
splat_key: dnd2024:equipment/padded_armor
splat_name: Padded Armor
splat_origin:
  key: dnd2024:reference/pages/092
splat_sources:
- title: SRD 5.2.1
  locator: pp. 92
  url: https://www.dndbeyond.com/srd
splat_item:
  armor:
    category: light armor
    stealth_disadvantage: true
    base_ac: 11
    add_dexterity_modifier: true
  price:
    value: 5
    unit: gp
  mass:
    value: 8.0
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
  text: '# Padded Armor Source table, SRD 5.2.1 page 92. { "armor": { "category": "light armor", "stealth_disadvantage":
    true, "base_ac": 11, "add_dexterity_modifier": true }, "price": { "value": 5, "unit": "gp" }, "mass":
    { "value": 8.0, "unit": "lb" }, "category": "armor" }'
  sha256: 3cd89e8420d0bc0d3dae85a34e3b03c0560b199d1af42e9b75a405f177a01ab0
  status: needs_semantic_conversion
---

# Padded Armor

Source table, SRD 5.2.1 page 92.

{
  "armor": {
    "category": "light armor",
    "stealth_disadvantage": true,
    "base_ac": 11,
    "add_dexterity_modifier": true
  },
  "price": {
    "value": 5,
    "unit": "gp"
  },
  "mass": {
    "value": 8.0,
    "unit": "lb"
  },
  "category": "armor"
}
