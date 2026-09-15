---
splat_schema: 1
splat_id: 25cd7903-6852-5d19-a127-ed37e95b775a
splat_kind: item
splat_key: dnd2024:equipment/blowgun
splat_name: Blowgun
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: '1'
    type: piercing
  properties_text: Ammunition (Range 25/100; Needle), Loading
  mastery: vex
  weapon_category: martial ranged
  price:
    value: 10
    unit: gp
  mass:
    value: 1.0
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
  text: '# Blowgun Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1", "type": "piercing"
    }, "properties_text": "Ammunition (Range 25/100; Needle), Loading", "mastery": "vex", "weapon_category":
    "martial ranged", "price": { "value": 10, "unit": "gp" }, "mass": { "value": 1.0, "unit": "lb" },
    "category": "weapon" }'
  sha256: 37085ebcb1694eb1d73213ed817357fc4b6095544278eab0b32bc9cd73d60fd8
  status: needs_semantic_conversion
---

# Blowgun

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1",
    "type": "piercing"
  },
  "properties_text": "Ammunition (Range 25/100; Needle), Loading",
  "mastery": "vex",
  "weapon_category": "martial ranged",
  "price": {
    "value": 10,
    "unit": "gp"
  },
  "mass": {
    "value": 1.0,
    "unit": "lb"
  },
  "category": "weapon"
}
