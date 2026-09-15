---
splat_schema: 1
splat_id: 130bc64f-94ba-5ac3-bc03-c7fe74e42bc3
splat_kind: item
splat_key: dnd2024:equipment/longbow
splat_name: Longbow
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: 1d8
    type: piercing
  properties_text: Ammunition (Range 150/600; Arrow), Heavy, Two-Handed
  mastery: slow
  weapon_category: martial ranged
  price:
    value: 50
    unit: gp
  mass:
    value: 2.0
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
  text: '# Longbow Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d8", "type": "piercing"
    }, "properties_text": "Ammunition (Range 150/600; Arrow), Heavy, Two-Handed", "mastery": "slow", "weapon_category":
    "martial ranged", "price": { "value": 50, "unit": "gp" }, "mass": { "value": 2.0, "unit": "lb" },
    "category": "weapon" }'
  sha256: 2fae0dd0cf45c7a4ed3bcdc8963e271a148fc231d22dce3b4c65ae98f8fa1455
  status: needs_semantic_conversion
---

# Longbow

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d8",
    "type": "piercing"
  },
  "properties_text": "Ammunition (Range 150/600; Arrow), Heavy, Two-Handed",
  "mastery": "slow",
  "weapon_category": "martial ranged",
  "price": {
    "value": 50,
    "unit": "gp"
  },
  "mass": {
    "value": 2.0,
    "unit": "lb"
  },
  "category": "weapon"
}
