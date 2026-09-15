---
splat_schema: 1
splat_id: ae6847a4-e9e0-513b-b60d-6ea97b144ca6
splat_kind: spell
splat_key: dnd2024:spells/protection_from_energy
splat_name: Protection from Energy
splat_origin:
  key: dnd2024:reference/spell/protection_from_energy
splat_sources:
- title: SRD 5.2.1
  locator: pp. 157
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: abjuration
  classes:
  - cleric
  - druid
  - ranger
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 1 hour
  casting_time: Action
  ritual: false
  range:
    mode: touch
splat_rule:
  mechanic: dnd2024:spells/protection_from_energy
  implementation: dnd2024:spell_protection_from_energy
  parameters:
    targeting:
      kind: creature
      count: 1
      willing: true
      touch: true
    resistance:
      choose_count: 1
      damage_types:
      - acid
      - cold
      - fire
      - lightning
      - thunder
      duration: spell_duration
    spell_lifetime:
      concentration: true
      maximum:
        value: 1
        unit: hour
splat_contracts:
- id: dnd2024:spell_protection_from_energy
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    targets:
      type: participant_list
      required: true
  outputs:
  - effects
  - damage_results
  events:
  - spell.cast_completed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - damage
splat_conversion:
  status: specified
  category: spell
  source_sha256: 976b5375e1dac1290af23235e6dda78a42011b8744ff8dcd0c8da62df565437a
  source_path: builtin/srd-5.2.1/spell/protection_from_energy.md
  pages:
  - 157
  engine_gaps: &id001
  - dnd2024:spell_protection_from_energy
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: d5a41e2d1301ff1e96f1115cb63e30493157a40349cb3dcf472ad3ca5a1e9080
splat_clauses:
- key: source_rules
  text: '# Protection from Energy Level 3 Abjuration (Cleric, Druid, Ranger, Sorcerer, Wizard) Casting
    Time: Action Range: Touch Components: V, S Duration: Concentration, up to 1 hour For the duration,
    the willing creature you touch has Resistance to one damage type of your choice: Acid, Cold, Fire,
    Lightning, or Thunder.'
  sha256: bcfa76f7980b74e7a9f371d7c314b6fd122d973b3ca6bac5d7a16758e59483e5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Protection from Energy

<!-- SRD page 157 -->

Level 3 Abjuration (Cleric, Druid, Ranger, Sorcerer,\
Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S\
Duration: Concentration, up to 1 hour\
For the duration, the willing creature you touch has\
Resistance to one damage type of your choice: Acid,\
Cold, Fire, Lightning, or Thunder.\

Original layout: [[../../srd-5.2.1/pages/157|page 157]].
