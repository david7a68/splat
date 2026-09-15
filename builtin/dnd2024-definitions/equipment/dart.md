---
splat_schema: 1
splat_id: 2f94f18b-c79b-581d-85d2-0ad46fd926a3
splat_kind: item
splat_key: dnd2024:equipment/dart
splat_name: Dart
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: 1d4
    type: piercing
  properties_text: Finesse, Thrown (Range 20/60)
  mastery: vex
  weapon_category: simple ranged
  price:
    value: 5
    unit: cp
  mass:
    value: 0.25
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
  text: '# Dart Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d4", "type": "piercing"
    }, "properties_text": "Finesse, Thrown (Range 20/60)", "mastery": "vex", "weapon_category": "simple
    ranged", "price": { "value": 5, "unit": "cp" }, "mass": { "value": 0.25, "unit": "lb" }, "category":
    "weapon" }'
  sha256: c282e7a25ae029f86593a9c79d4b525b22befa885a7b8d98b389ababedd59c45
  status: needs_semantic_conversion
---

# Dart

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d4",
    "type": "piercing"
  },
  "properties_text": "Finesse, Thrown (Range 20/60)",
  "mastery": "vex",
  "weapon_category": "simple ranged",
  "price": {
    "value": 5,
    "unit": "cp"
  },
  "mass": {
    "value": 0.25,
    "unit": "lb"
  },
  "category": "weapon"
}
