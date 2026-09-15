---
splat_schema: 1
splat_id: 951565b3-3df2-56cf-9aa0-e5cba90132d7
splat_kind: item
splat_key: dnd2024:equipment/sling
splat_name: Sling
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: 1d4
    type: bludgeoning
  properties_text: Ammunition (Range 30/120; Bullet)
  mastery: slow
  weapon_category: simple ranged
  price:
    value: 1
    unit: sp
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
  text: '# Sling Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d4", "type": "bludgeoning"
    }, "properties_text": "Ammunition (Range 30/120; Bullet)", "mastery": "slow", "weapon_category": "simple
    ranged", "price": { "value": 1, "unit": "sp" }, "category": "weapon" }'
  sha256: 623cbdb72966c323a5cbd629582355e5d84182c1beb79841afa7a66196b73316
  status: needs_semantic_conversion
---

# Sling

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d4",
    "type": "bludgeoning"
  },
  "properties_text": "Ammunition (Range 30/120; Bullet)",
  "mastery": "slow",
  "weapon_category": "simple ranged",
  "price": {
    "value": 1,
    "unit": "sp"
  },
  "category": "weapon"
}
