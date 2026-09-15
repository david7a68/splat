---
splat_schema: 1
splat_id: 72531fa2-5662-5996-89fd-07c5b8af784c
splat_kind: rule
splat_key: dnd2024:glossary/hit_points
splat_name: Hit Points
splat_origin:
  key: dnd2024:reference/glossary/hit_points
splat_sources:
- title: SRD 5.2.1
  locator: pp. 183
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/hit_points
  parameters:
    eligible:
    - creature
    - object
    minimum: 0
    maximum:
      var: hit_point_maximum
    damage_operation: subtract
    healing_operation: add
  implementation: dnd2024:glossary_hit_points
splat_contracts:
- id: dnd2024:glossary_hit_points
  inputs:
    subject:
      type: object
      required: true
    change:
      type: integer
      required: true
  outputs:
  - clamped_hit_points
  events:
  - hit_points.changed
  capabilities:
  - hit_points
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 9d0b3d995d5fb4d5e872c14238b8b562df0e1969b5bc17272fe4dac57d02aadb
  source_path: builtin/srd-5.2.1/glossary/hit_points.md
  pages:
  - 183
  engine_gaps: &id001
  - dnd2024:glossary_hit_points
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 4b6da0e4a56dd1dc6e49287f5aeb35215c374c744fc7c4c7f77f5dfd3f4b7d6f
splat_clauses:
- key: source_rules
  text: '# Hit Points Hit Points (HP) are a measure of how difficult it is to kill or destroy a creature
    or an object. Damage reduces Hit Points, and healing restores them. You can’t have more Hit Points
    than your Hit Point maximum, and you can’t have less than 0. See also “Breaking Objects” and “Playing
    the Game” (“Damage and Healing”).'
  sha256: 72b9469c130703d3799469654b6b6f146d04e619330d62a353ace0eafa9b4a9e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Hit Points

<!-- SRD page 183 -->

Hit Points (HP) are a measure of how difficult it is\
to kill or destroy a creature or an object. Damage\
reduces Hit Points, and healing restores them.\
You can’t have more Hit Points than your Hit Point\
maximum, and you can’t have less than 0. See also\
“Breaking Objects” and “Playing the Game” (“Dam-\
age and Healing”).\

Original layout: [[../../srd-5.2.1/pages/183|page 183]].
