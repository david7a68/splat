---
splat_schema: 1
splat_id: 66dd5ecc-0fb7-5919-ad05-d867cb25d41f
splat_kind: item
splat_key: dnd2024:equipment/mace
splat_name: Mace
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: 1d6
    type: bludgeoning
  properties_text: —
  mastery: sap
  weapon_category: simple melee
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
  text: '# Mace Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d6", "type": "bludgeoning"
    }, "properties_text": "—", "mastery": "sap", "weapon_category": "simple melee", "price": { "value":
    5, "unit": "gp" }, "mass": { "value": 4.0, "unit": "lb" }, "category": "weapon" }'
  sha256: 64a084de497191307f5193d9b593eaaf801750d30b0642589f72dbddc1e6fb4b
  status: needs_semantic_conversion
---

# Mace

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d6",
    "type": "bludgeoning"
  },
  "properties_text": "—",
  "mastery": "sap",
  "weapon_category": "simple melee",
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
