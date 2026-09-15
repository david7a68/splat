---
splat_schema: 1
splat_id: 6815bcf2-fb7e-51d4-9c32-5fb0bf8105ef
splat_kind: rule
splat_key: dnd2024:rules/weapons_push
splat_name: Push
splat_origin:
  key: dnd2024:reference/pages/090
splat_sources:
- title: SRD 5.2.1
  locator: pp. 90
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/weapons_push
  parameters:
    trigger:
      weapon_hit: true
      target_kind: creature
    optional: true
    target_size_max: large
    push:
      distance_ft_min: 0
      distance_ft_max: 10
      direction: straight_away_from_attacker
  implementation: dnd2024:rules_weapons_push
splat_contracts:
- id: dnd2024:rules_weapons_push
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - effects
  events:
  - attack.hit
  capabilities:
  - weapon_mastery
  - forced_movement
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 90
  engine_gaps: &id001
  - dnd2024:rules_weapons_push
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 653aaf78412ea6bff46d604c0f2bb6bf99fc7dff96b3f1695f8915309fbe92a0
  parent_name: Weapons
  line_span:
    first: 7793
    count: 4
splat_clauses:
- key: source_rules
  text: '# Push If you hit a creature with this weapon, you can push the creature up to 10 feet straight
    away from yourself if it is Large or smaller.'
  sha256: 4b401f89c26b8aa09547f0925e9e22954449c89c4667f0a1e2421dda2d804873
  status: contract_specified
splat_requires_mechanics: *id001
---

# Push

<!-- SRD page 90 -->

If you hit a creature with this weapon, you can push\
the creature up to 10 feet straight away from your-\
self if it is Large or smaller.\
