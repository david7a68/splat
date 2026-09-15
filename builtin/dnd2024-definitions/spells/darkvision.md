---
splat_schema: 1
splat_id: a3811a12-0bd8-5a41-9bc5-cd58e5d0bd3d
splat_kind: spell
splat_key: dnd2024:spells/darkvision
splat_name: Darkvision
splat_origin:
  key: dnd2024:reference/spell/darkvision
splat_sources:
- title: SRD 5.2.1
  locator: pp. 122
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: transmutation
  classes:
  - druid
  - ranger
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 8 hours
  casting_time: Action
  ritual: false
  range:
    mode: touch
  material:
    description: a dried carrot
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/darkvision
  implementation: dnd2024:spell_darkvision
  parameters:
    spell_lifetime:
      concentration: false
      maximum_seconds: 28800
    target:
      kind: creature
      willing: true
      touch: true
    sense:
      kind: darkvision
      range_feet: 150
splat_contracts:
- id: dnd2024:spell_darkvision
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    target:
      type: participant
      required: true
  outputs:
  - effects
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
splat_conversion:
  status: specified
  category: spell
  source_sha256: 29130598ce0572fd4d5053dc56184bcd4277adca34ec2b31905491fb6dc3aa5e
  source_path: builtin/srd-5.2.1/spell/darkvision.md
  pages:
  - 122
  engine_gaps: &id001
  - dnd2024:spell_darkvision
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: 94569fec76b2ad6cbbd476bff3d4004a74ce39e385c95585b48585377b19c858
splat_clauses:
- key: source_rules
  text: '# Darkvision Level 2 Transmutation (Druid, Ranger, Sorcerer, Wizard) Casting Time: Action Range:
    Touch Components: V, S, M (a dried carrot) Duration: 8 hours For the duration, a willing creature
    you touch has Darkvision with a range of 150 feet.'
  sha256: f5b368ea4aeb3e4d1d30753849774c8640cc2aaab966d0239e42fe03aba74c64
  status: contract_specified
splat_requires_mechanics: *id001
---

# Darkvision

<!-- SRD page 122 -->

Level 2 Transmutation (Druid, Ranger, Sorcerer,\
Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (a dried carrot)\
Duration: 8 hours\
For the duration, a willing creature you touch has\
Darkvision with a range of 150 feet.\

Original layout: [[../../srd-5.2.1/pages/122|page 122]].
