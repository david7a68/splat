---
splat_schema: 1
splat_id: a151446c-de15-554e-87c3-b8d5ded116c4
splat_kind: item
splat_key: dnd2024:equipment/light_crossbow
splat_name: Light Crossbow
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
  properties_text: Ammunition (Range 80/320; Bolt), Loading, Two-Handed
  mastery: slow
  weapon_category: simple ranged
  price:
    value: 25
    unit: gp
  mass:
    value: 5.0
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
  text: '# Light Crossbow Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d8", "type":
    "piercing" }, "properties_text": "Ammunition (Range 80/320; Bolt), Loading, Two-Handed", "mastery":
    "slow", "weapon_category": "simple ranged", "price": { "value": 25, "unit": "gp" }, "mass": { "value":
    5.0, "unit": "lb" }, "category": "weapon" }'
  sha256: 1bdf9da11b7a81c66ceee844ba450d569cd72e6c2f6428950a5027980b62da35
  status: needs_semantic_conversion
---

# Light Crossbow

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d8",
    "type": "piercing"
  },
  "properties_text": "Ammunition (Range 80/320; Bolt), Loading, Two-Handed",
  "mastery": "slow",
  "weapon_category": "simple ranged",
  "price": {
    "value": 25,
    "unit": "gp"
  },
  "mass": {
    "value": 5.0,
    "unit": "lb"
  },
  "category": "weapon"
}
