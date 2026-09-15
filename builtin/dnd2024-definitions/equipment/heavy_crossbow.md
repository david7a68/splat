---
splat_schema: 1
splat_id: 2617805c-1c9e-5706-b9e2-3c769e36b321
splat_kind: item
splat_key: dnd2024:equipment/heavy_crossbow
splat_name: Heavy Crossbow
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
  properties_text: Ammunition (Range 100/400; Bolt), Heavy, Loading, Two-Handed
  mastery: push
  weapon_category: martial ranged
  price:
    value: 50
    unit: gp
  mass:
    value: 18.0
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
  text: '# Heavy Crossbow Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d10", "type":
    "piercing" }, "properties_text": "Ammunition (Range 100/400; Bolt), Heavy, Loading, Two-Handed", "mastery":
    "push", "weapon_category": "martial ranged", "price": { "value": 50, "unit": "gp" }, "mass": { "value":
    18.0, "unit": "lb" }, "category": "weapon" }'
  sha256: 8597294fb3f9f22ab99ce60ffc796e65ff35239ba251073e5b327df0a1608b0f
  status: needs_semantic_conversion
---

# Heavy Crossbow

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d10",
    "type": "piercing"
  },
  "properties_text": "Ammunition (Range 100/400; Bolt), Heavy, Loading, Two-Handed",
  "mastery": "push",
  "weapon_category": "martial ranged",
  "price": {
    "value": 50,
    "unit": "gp"
  },
  "mass": {
    "value": 18.0,
    "unit": "lb"
  },
  "category": "weapon"
}
