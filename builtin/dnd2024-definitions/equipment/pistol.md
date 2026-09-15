---
splat_schema: 1
splat_id: ba85cb49-8b93-51b2-aa6d-cb0355c8bd1b
splat_kind: item
splat_key: dnd2024:equipment/pistol
splat_name: Pistol
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: 1d10
    type: piercing
  properties_text: Ammunition (Range 30/90; Bullet), Loading
  mastery: vex
  weapon_category: martial ranged
  price:
    value: 250
    unit: gp
  mass:
    value: 3.0
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
  text: '# Pistol Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d10", "type": "piercing"
    }, "properties_text": "Ammunition (Range 30/90; Bullet), Loading", "mastery": "vex", "weapon_category":
    "martial ranged", "price": { "value": 250, "unit": "gp" }, "mass": { "value": 3.0, "unit": "lb" },
    "category": "weapon" }'
  sha256: d9e2d1e1ff61317ac7ace1ab2f436be739b642853e650f4c3f1136aa29645c90
  status: needs_semantic_conversion
---

# Pistol

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d10",
    "type": "piercing"
  },
  "properties_text": "Ammunition (Range 30/90; Bullet), Loading",
  "mastery": "vex",
  "weapon_category": "martial ranged",
  "price": {
    "value": 250,
    "unit": "gp"
  },
  "mass": {
    "value": 3.0,
    "unit": "lb"
  },
  "category": "weapon"
}
