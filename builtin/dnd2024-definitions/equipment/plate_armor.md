---
splat_schema: 1
splat_id: 866cfefa-c2f4-5bb6-bfca-50429ed4d20f
splat_kind: item
splat_key: dnd2024:equipment/plate_armor
splat_name: Plate Armor
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
    base_ac: 18
    add_dexterity_modifier: false
    strength_minimum: 15
  price:
    value: 1500
    unit: gp
  mass:
    value: 65.0
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
  text: '# Plate Armor Source table, SRD 5.2.1 page 92. { "armor": { "category": "heavy armor", "stealth_disadvantage":
    true, "base_ac": 18, "add_dexterity_modifier": false, "strength_minimum": 15 }, "price": { "value":
    1500, "unit": "gp" }, "mass": { "value": 65.0, "unit": "lb" }, "category": "armor" }'
  sha256: 4c5cfc4c7b53a791eba8859f292f1103df84a8061c446ff874f28fb45f143a25
  status: needs_semantic_conversion
---

# Plate Armor

Source table, SRD 5.2.1 page 92.

{
  "armor": {
    "category": "heavy armor",
    "stealth_disadvantage": true,
    "base_ac": 18,
    "add_dexterity_modifier": false,
    "strength_minimum": 15
  },
  "price": {
    "value": 1500,
    "unit": "gp"
  },
  "mass": {
    "value": 65.0,
    "unit": "lb"
  },
  "category": "armor"
}
