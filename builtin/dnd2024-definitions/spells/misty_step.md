---
splat_schema: 1
splat_id: 30bf5254-82bb-570e-b6c6-79897d4fcbf0
splat_kind: spell
splat_key: dnd2024:spells/misty_step
splat_name: Misty Step
splat_origin:
  key: dnd2024:reference/spell/misty_step
splat_sources:
- title: SRD 5.2.1
  locator: pp. 150
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: conjuration
  classes:
  - sorcerer
  - warlock
  - wizard
  components:
  - verbal
  concentration: false
  duration: Instantaneous
  casting_time: Bonus Action
  ritual: false
  range:
    mode: self
splat_rule:
  mechanic: dnd2024:spells/misty_step
  implementation: dnd2024:spell_misty_step
  parameters:
    teleport:
      subject: caster
      maximum_feet: 30
      destination_unoccupied: true
      destination_visible: true
splat_contracts:
- id: dnd2024:spell_misty_step
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    locations:
      required: false
      type: location_list
    campaign:
      required: false
      type: campaign
    choices:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - spell.cast_completed
  - movement.completed
  capabilities:
  - spellcasting
  - effects
  - spatial_relationships
splat_conversion:
  status: specified
  category: spell
  source_sha256: 7b7bbf4804d3c9a194a602e1eb5c3d5aab73cbfe018785f913b6b187fc8ac454
  source_path: builtin/srd-5.2.1/spell/misty_step.md
  pages:
  - 150
  engine_gaps: &id001
  - dnd2024:spell_misty_step
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 6f8a6cc354b23c8b0e0ce9db122745075e5fbc78f14062e884efb252092382bf
splat_clauses:
- key: source_rules
  text: '# Misty Step Level 2 Conjuration (Sorcerer, Warlock, Wizard) Casting Time: Bonus Action Range:
    Self Components: V Duration: Instantaneous Briefly surrounded by silvery mist, you teleport up to
    30 feet to an unoccupied space you can see.'
  sha256: 64b5527d0c7d84ec66b734a317c49940e918d2a69c43c426f56037d99db67130
  status: contract_specified
splat_requires_mechanics: *id001
---

# Misty Step

<!-- SRD page 150 -->

Level 2 Conjuration (Sorcerer, Warlock, Wizard)\
Casting Time: Bonus Action\
Range: Self\
Components: V\
Duration: Instantaneous\
Briefly surrounded by silvery mist, you teleport up\
to 30 feet to an unoccupied space you can see.\

Original layout: [[../../srd-5.2.1/pages/150|page 150]].
