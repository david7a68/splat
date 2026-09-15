---
splat_schema: 1
splat_id: f626022b-7821-5165-9050-3b82bc627e72
splat_kind: rule
splat_key: dnd2024:glossary/temporary_hit_points
splat_name: Temporary Hit Points
splat_origin:
  key: dnd2024:reference/glossary/temporary_hit_points
splat_sources:
- title: SRD 5.2.1
  locator: pp. 190
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/temporary_hit_points
  parameters:
    resource: temporary_hit_points
    damage_absorption:
      before: real_hit_points
      amount:
        op: min
        args:
        - var: temporary_hit_points
        - var: incoming_damage
    grant_source: effect_defined
  implementation: dnd2024:glossary_temporary_hit_points
splat_contracts:
- id: dnd2024:glossary_temporary_hit_points
  inputs:
    subject:
      type: participant
      required: true
    incoming_damage:
      type: integer
      required: true
  outputs:
  - temporary_hit_points_spent
  - remaining_damage
  events:
  - damage.pending
  capabilities:
  - hit_points
  - damage
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 8395cc584f3015d0a3daa39f97a603393fbb95b864a42162b3d4e54389dc7fed
  source_path: builtin/srd-5.2.1/glossary/temporary_hit_points.md
  pages:
  - 190
  engine_gaps: &id001
  - dnd2024:glossary_temporary_hit_points
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 4895c50e9e3e3d416a13643625f0952450caa56eb392e3497e038eef4216b137
splat_clauses:
- key: source_rules
  text: '# Temporary Hit Points Temporary Hit Points are granted by certain effects and act as a buffer
    against losing real Hit Points. See also “Playing the Game” (“Damage and Healing”).'
  sha256: 6e81b365300b3c594570855113a886c061ccf325e751900fa2ff09913ace5891
  status: contract_specified
splat_requires_mechanics: *id001
---

# Temporary Hit Points

<!-- SRD page 190 -->

Temporary Hit Points are granted by certain effects\
and act as a buffer against losing real Hit Points. See\
also “Playing the Game” (“Damage and Healing”).\

Original layout: [[../../srd-5.2.1/pages/190|page 190]].
