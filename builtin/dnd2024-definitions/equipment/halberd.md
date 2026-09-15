---
splat_schema: 1
splat_id: 88cc4a73-b348-5fe9-8229-54b9dbbecea7
splat_kind: item
splat_key: dnd2024:equipment/halberd
splat_name: Halberd
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: 1d10
    type: slashing
  properties_text: Heavy, Reach, Two-Handed
  mastery: cleave
  weapon_category: martial melee
  price:
    value: 20
    unit: gp
  mass:
    value: 6.0
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
  text: '# Halberd Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d10", "type": "slashing"
    }, "properties_text": "Heavy, Reach, Two-Handed", "mastery": "cleave", "weapon_category": "martial
    melee", "price": { "value": 20, "unit": "gp" }, "mass": { "value": 6.0, "unit": "lb" }, "category":
    "weapon" }'
  sha256: be27d90e0f901779d9ed58684ba8a4b9f69ac39a007e664e9cc04366a530e413
  status: needs_semantic_conversion
---

# Halberd

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d10",
    "type": "slashing"
  },
  "properties_text": "Heavy, Reach, Two-Handed",
  "mastery": "cleave",
  "weapon_category": "martial melee",
  "price": {
    "value": 20,
    "unit": "gp"
  },
  "mass": {
    "value": 6.0,
    "unit": "lb"
  },
  "category": "weapon"
}
