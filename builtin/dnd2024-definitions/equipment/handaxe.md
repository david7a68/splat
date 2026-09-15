---
splat_schema: 1
splat_id: 34e4dda3-515b-589c-b53c-ff865efe81ce
splat_kind: item
splat_key: dnd2024:equipment/handaxe
splat_name: Handaxe
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
  properties_text: Light, Thrown (Range 20/60)
  mastery: vex
  weapon_category: simple melee
  price:
    value: 5
    unit: gp
  mass:
    value: 2.0
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
  text: '# Handaxe Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d6", "type": "slashing"
    }, "properties_text": "Light, Thrown (Range 20/60)", "mastery": "vex", "weapon_category": "simple
    melee", "price": { "value": 5, "unit": "gp" }, "mass": { "value": 2.0, "unit": "lb" }, "category":
    "weapon" }'
  sha256: 27c90d74ce0eb5e1636e76e99508d198a340c0624eb530cc83be4a0bdf965952
  status: needs_semantic_conversion
---

# Handaxe

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d6",
    "type": "slashing"
  },
  "properties_text": "Light, Thrown (Range 20/60)",
  "mastery": "vex",
  "weapon_category": "simple melee",
  "price": {
    "value": 5,
    "unit": "gp"
  },
  "mass": {
    "value": 2.0,
    "unit": "lb"
  },
  "category": "weapon"
}
