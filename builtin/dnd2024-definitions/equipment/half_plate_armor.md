---
splat_schema: 1
splat_id: af613d48-01e3-585f-8f80-6838767ee644
splat_kind: item
splat_key: dnd2024:equipment/half_plate_armor
splat_name: Half Plate Armor
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
    base_ac: 15
    add_dexterity_modifier: true
    dexterity_modifier_max: 2
  price:
    value: 750
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
  text: '# Half Plate Armor Source table, SRD 5.2.1 page 92. { "armor": { "category": "medium armor",
    "stealth_disadvantage": true, "base_ac": 15, "add_dexterity_modifier": true, "dexterity_modifier_max":
    2 }, "price": { "value": 750, "unit": "gp" }, "mass": { "value": 40.0, "unit": "lb" }, "category":
    "armor" }'
  sha256: 3cbd6bc1e933b7b9c5bbd939e7d44f474b558a4452fb0968107c2462622b1512
  status: needs_semantic_conversion
---

# Half Plate Armor

Source table, SRD 5.2.1 page 92.

{
  "armor": {
    "category": "medium armor",
    "stealth_disadvantage": true,
    "base_ac": 15,
    "add_dexterity_modifier": true,
    "dexterity_modifier_max": 2
  },
  "price": {
    "value": 750,
    "unit": "gp"
  },
  "mass": {
    "value": 40.0,
    "unit": "lb"
  },
  "category": "armor"
}
