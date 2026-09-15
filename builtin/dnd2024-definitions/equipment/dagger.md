---
splat_schema: 1
splat_id: 3302150c-7356-5633-9215-20d96c893348
splat_kind: item
splat_key: dnd2024:equipment/dagger
splat_name: Dagger
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
  properties_text: Finesse, Light, Thrown (Range 20/60)
  mastery: nick
  weapon_category: simple melee
  price:
    value: 2
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
  text: '# Dagger Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d4", "type": "piercing"
    }, "properties_text": "Finesse, Light, Thrown (Range 20/60)", "mastery": "nick", "weapon_category":
    "simple melee", "price": { "value": 2, "unit": "gp" }, "mass": { "value": 1.0, "unit": "lb" }, "category":
    "weapon" }'
  sha256: cffc1783049b16e9bf8688e1ff506dd47aa0fb794bba453373a66ad8a638286e
  status: needs_semantic_conversion
---

# Dagger

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d4",
    "type": "piercing"
  },
  "properties_text": "Finesse, Light, Thrown (Range 20/60)",
  "mastery": "nick",
  "weapon_category": "simple melee",
  "price": {
    "value": 2,
    "unit": "gp"
  },
  "mass": {
    "value": 1.0,
    "unit": "lb"
  },
  "category": "weapon"
}
