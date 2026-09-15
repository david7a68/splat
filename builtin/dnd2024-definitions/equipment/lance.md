---
splat_schema: 1
splat_id: 0b572249-2f3d-5e90-a2b7-1aa71dfef47d
splat_kind: item
splat_key: dnd2024:equipment/lance
splat_name: Lance
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
  properties_text: Heavy, Reach, Two-Handed (unless mounted)
  mastery: topple
  weapon_category: martial melee
  price:
    value: 10
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
  text: '# Lance Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d10", "type": "piercing"
    }, "properties_text": "Heavy, Reach, Two-Handed (unless mounted)", "mastery": "topple", "weapon_category":
    "martial melee", "price": { "value": 10, "unit": "gp" }, "mass": { "value": 6.0, "unit": "lb" }, "category":
    "weapon" }'
  sha256: afc75d9ddcd43088873b82d96778c7cf99ad2a7e8b9b8f34c32262121cf6a483
  status: needs_semantic_conversion
---

# Lance

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d10",
    "type": "piercing"
  },
  "properties_text": "Heavy, Reach, Two-Handed (unless mounted)",
  "mastery": "topple",
  "weapon_category": "martial melee",
  "price": {
    "value": 10,
    "unit": "gp"
  },
  "mass": {
    "value": 6.0,
    "unit": "lb"
  },
  "category": "weapon"
}
