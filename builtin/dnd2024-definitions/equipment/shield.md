---
splat_schema: 1
splat_id: 3c73c0a9-979d-5f24-abb3-9d5522c14f78
splat_kind: item
splat_key: dnd2024:equipment/shield
splat_name: Shield
splat_origin:
  key: dnd2024:reference/pages/092
splat_sources:
- title: SRD 5.2.1
  locator: pp. 92
  url: https://www.dndbeyond.com/srd
splat_item:
  armor:
    category: shield
    stealth_disadvantage: false
    ac_bonus: 2
  price:
    value: 10
    unit: gp
  mass:
    value: 6.0
    unit: lb
  category: armor
splat_conversion:
  status: partial
  category: equipment
  source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
  source_path: builtin/srd-5.2.1/source.txt
  pages:
  - 92
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Shield Source table, SRD 5.2.1 page 92. { "armor": { "category": "shield", "stealth_disadvantage":
    false, "ac_bonus": 2 }, "price": { "value": 10, "unit": "gp" }, "mass": { "value": 6.0, "unit": "lb"
    }, "category": "armor" }'
  sha256: bc6077240d365f6389d54c5823fd6b894f419c2b2307ea7838b17f8ee64f460c
  status: needs_semantic_conversion
---

# Shield

Source table, SRD 5.2.1 page 92.

{
  "armor": {
    "category": "shield",
    "stealth_disadvantage": false,
    "ac_bonus": 2
  },
  "price": {
    "value": 10,
    "unit": "gp"
  },
  "mass": {
    "value": 6.0,
    "unit": "lb"
  },
  "category": "armor"
}
