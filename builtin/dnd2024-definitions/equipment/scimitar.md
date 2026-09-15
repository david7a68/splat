---
splat_schema: 1
splat_id: 37150ba9-bad3-505b-9d1b-70afd910b940
splat_kind: item
splat_key: dnd2024:equipment/scimitar
splat_name: Scimitar
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: 1d6
    type: slashing
  properties_text: Finesse, Light
  mastery: nick
  weapon_category: martial melee
  price:
    value: 25
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
  text: '# Scimitar Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d6", "type": "slashing"
    }, "properties_text": "Finesse, Light", "mastery": "nick", "weapon_category": "martial melee", "price":
    { "value": 25, "unit": "gp" }, "mass": { "value": 3.0, "unit": "lb" }, "category": "weapon" }'
  sha256: 1367fe3632add25fdaebc281e20d0a67d3de8a67ac9d6449296972494752fafa
  status: needs_semantic_conversion
---

# Scimitar

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d6",
    "type": "slashing"
  },
  "properties_text": "Finesse, Light",
  "mastery": "nick",
  "weapon_category": "martial melee",
  "price": {
    "value": 25,
    "unit": "gp"
  },
  "mass": {
    "value": 3.0,
    "unit": "lb"
  },
  "category": "weapon"
}
