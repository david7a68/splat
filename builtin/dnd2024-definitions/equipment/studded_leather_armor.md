---
splat_schema: 1
splat_id: 899853f5-26b9-5157-b5ae-f699c05369d0
splat_kind: item
splat_key: dnd2024:equipment/studded_leather_armor
splat_name: Studded Leather Armor
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
    base_ac: 12
    add_dexterity_modifier: true
  price:
    value: 45
    unit: gp
  mass:
    value: 13.0
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
  text: '# Studded Leather Armor Source table, SRD 5.2.1 page 92. { "armor": { "category": "light armor",
    "stealth_disadvantage": false, "base_ac": 12, "add_dexterity_modifier": true }, "price": { "value":
    45, "unit": "gp" }, "mass": { "value": 13.0, "unit": "lb" }, "category": "armor" }'
  sha256: 2f86707200ccfe7fc6d973261d1778d1f581796bbad4cce8ce0e1fddb9b6650e
  status: needs_semantic_conversion
---

# Studded Leather Armor

Source table, SRD 5.2.1 page 92.

{
  "armor": {
    "category": "light armor",
    "stealth_disadvantage": false,
    "base_ac": 12,
    "add_dexterity_modifier": true
  },
  "price": {
    "value": 45,
    "unit": "gp"
  },
  "mass": {
    "value": 13.0,
    "unit": "lb"
  },
  "category": "armor"
}
