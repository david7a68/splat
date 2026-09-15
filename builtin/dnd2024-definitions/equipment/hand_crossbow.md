---
splat_schema: 1
splat_id: 2a9b7238-03dd-58b6-ab94-c4e60b9b8b5d
splat_kind: item
splat_key: dnd2024:equipment/hand_crossbow
splat_name: Hand Crossbow
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: 1d6
    type: piercing
  properties_text: Ammunition (Range 30/120; Bolt), Light, Loading
  mastery: vex
  weapon_category: martial ranged
  price:
    value: 75
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
  text: '# Hand Crossbow Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d6", "type": "piercing"
    }, "properties_text": "Ammunition (Range 30/120; Bolt), Light, Loading", "mastery": "vex", "weapon_category":
    "martial ranged", "price": { "value": 75, "unit": "gp" }, "mass": { "value": 3.0, "unit": "lb" },
    "category": "weapon" }'
  sha256: 83a3fab30739786d33d591a817656bc8bda77c6d01af2f541f6c16a391ddcd55
  status: needs_semantic_conversion
---

# Hand Crossbow

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d6",
    "type": "piercing"
  },
  "properties_text": "Ammunition (Range 30/120; Bolt), Light, Loading",
  "mastery": "vex",
  "weapon_category": "martial ranged",
  "price": {
    "value": 75,
    "unit": "gp"
  },
  "mass": {
    "value": 3.0,
    "unit": "lb"
  },
  "category": "weapon"
}
