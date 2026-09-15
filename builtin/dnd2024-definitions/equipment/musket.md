---
splat_schema: 1
splat_id: 8f28f780-21f0-5845-824e-5145c40d7fe5
splat_kind: item
splat_key: dnd2024:equipment/musket
splat_name: Musket
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: 1d12
    type: piercing
  properties_text: Ammunition (Range 40/120; Bullet), Loading, Two-Handed
  mastery: slow
  weapon_category: martial ranged
  price:
    value: 500
    unit: gp
  mass:
    value: 10.0
    unit: lb
  category: weapon
splat_conversion:
  status: partial
  category: equipment
  source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
  source_path: builtin/srd-5.2.1/source.txt
  pages:
  - 91
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Musket Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d12", "type": "piercing"
    }, "properties_text": "Ammunition (Range 40/120; Bullet), Loading, Two-Handed", "mastery": "slow",
    "weapon_category": "martial ranged", "price": { "value": 500, "unit": "gp" }, "mass": { "value": 10.0,
    "unit": "lb" }, "category": "weapon" }'
  sha256: fb98819000cade7a8d201d8aa243650d2e386926fdae51a15bf0941546bea0a6
  status: needs_semantic_conversion
---

# Musket

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d12",
    "type": "piercing"
  },
  "properties_text": "Ammunition (Range 40/120; Bullet), Loading, Two-Handed",
  "mastery": "slow",
  "weapon_category": "martial ranged",
  "price": {
    "value": 500,
    "unit": "gp"
  },
  "mass": {
    "value": 10.0,
    "unit": "lb"
  },
  "category": "weapon"
}
