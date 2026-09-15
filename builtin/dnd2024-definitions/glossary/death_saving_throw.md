---
splat_schema: 1
splat_id: 247390f9-a645-5c90-899a-a1976cf3cb49
splat_kind: rule
splat_key: dnd2024:glossary/death_saving_throw
splat_name: Death Saving Throw
splat_origin:
  key: dnd2024:reference/glossary/death_saving_throw
splat_sources:
- title: SRD 5.2.1
  locator: pp. 181
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/death_saving_throw
  parameters:
    trigger: own_turn_start
    subject_kind: player_character
    hit_points: 0
    required_test: death_saving_throw
    exception_rule:
      key: dnd2024:glossary/stable
  implementation: dnd2024:glossary_death_saving_throw
splat_contracts:
- id: dnd2024:glossary_death_saving_throw
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - death_save_required
  events:
  - turn.started
  capabilities:
  - death
  - checks
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 592b5ac973e56ccbdfc9a35320dc6511bf5fdd4aead728ad0a7b5b975d369036
  source_path: builtin/srd-5.2.1/glossary/death_saving_throw.md
  pages:
  - 181
  engine_gaps: &id001
  - dnd2024:glossary_death_saving_throw
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 8c44ccd359920648fab923b3a0f393b0aa094e37514c8b61944866a203f5c345
splat_clauses:
- key: source_rules
  text: '# Death Saving Throw A player character must make a Death Saving Throw (also called a Death Save)
    if they start their turn with 0 Hit Points. See also “Playing the Game” (“Damage and Healing”).'
  sha256: 91a985393841e98d10f203d3a25930bca42825674eeedd04badbb6b6ce662507
  status: contract_specified
splat_requires_mechanics: *id001
---

# Death Saving Throw

<!-- SRD page 181 -->

A player character must make a Death Saving\
Throw (also called a Death Save) if they start their\
turn with 0 Hit Points. See also “Playing the Game”\
(“Damage and Healing”).\

Original layout: [[../../srd-5.2.1/pages/181|page 181]].
