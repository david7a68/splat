---
splat_schema: 1
splat_id: 6c9dd52c-59b0-595d-8f70-19587d9c0690
splat_kind: item
splat_key: dnd2024:equipment/trident
splat_name: Trident
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
  properties_text: Thrown (Range 20/60), Versatile (1d10)
  mastery: topple
  weapon_category: martial melee
  price:
    value: 5
    unit: gp
  mass:
    value: 4.0
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
  text: '# Trident Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d8", "type": "piercing"
    }, "properties_text": "Thrown (Range 20/60), Versatile (1d10)", "mastery": "topple", "weapon_category":
    "martial melee", "price": { "value": 5, "unit": "gp" }, "mass": { "value": 4.0, "unit": "lb" }, "category":
    "weapon" }'
  sha256: 1b690ac20195da2eca99ce891b25c3829748ac4495b5926f0eed56016dd09776
  status: needs_semantic_conversion
---

# Trident

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d8",
    "type": "piercing"
  },
  "properties_text": "Thrown (Range 20/60), Versatile (1d10)",
  "mastery": "topple",
  "weapon_category": "martial melee",
  "price": {
    "value": 5,
    "unit": "gp"
  },
  "mass": {
    "value": 4.0,
    "unit": "lb"
  },
  "category": "weapon"
}
