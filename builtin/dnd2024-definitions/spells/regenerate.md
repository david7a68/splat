---
splat_schema: 1
splat_id: 4cac4ccd-a035-56ef-accb-deae22998694
splat_kind: spell
splat_key: dnd2024:spells/regenerate
splat_name: Regenerate
splat_origin:
  key: dnd2024:reference/spell/regenerate
splat_sources:
- title: SRD 5.2.1
  locator: pp. 158
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 7
  school: transmutation
  classes:
  - bard
  - cleric
  - druid
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 1 hour
  casting_time: 1 minute
  ritual: false
  range:
    mode: touch
  material:
    description: a prayer wheel
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/regenerate
  implementation: dnd2024:spell_regenerate
  parameters:
    targeting:
      kind: creature
      count: 1
      touch: true
    initial_healing:
      dice: 4d8
      add: 15
    ongoing:
      duration: spell_duration
      healing:
        event: target.turn_started
        amount: 1
      severed_part_regrowth_minutes: 2
    spell_lifetime:
      concentration: false
      maximum:
        value: 1
        unit: hour
splat_contracts:
- id: dnd2024:spell_regenerate
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
  - healing_results
  events:
  - spell.cast_completed
  - turn.started
  - time.elapsed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - healing
splat_conversion:
  status: specified
  category: spell
  source_sha256: 086f00824adcc857f17cb591652b676189219419ecdd3be41644371487be5d0c
  source_path: builtin/srd-5.2.1/spell/regenerate.md
  pages:
  - 158
  engine_gaps: &id001
  - dnd2024:spell_regenerate
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: e61498bbae75607907a4cdda43fed8e1336147a741ebf427ee8cdfbee9dd463c
splat_clauses:
- key: source_rules
  text: '# Regenerate Level 7 Transmutation (Bard, Cleric, Druid) Casting Time: 1 minute Range: Touch
    Components: V, S, M (a prayer wheel) Duration: 1 hour A creature you touch regains 4d8 + 15 Hit Points.
    For the duration, the target regains 1 Hit Point at the start of each of its turns, and any severed
    body parts regrow after 2 minutes.'
  sha256: f3a7b29cf2f9fc7134ffe931bb02eb3bcba54f9c70df3e5eaf7822d65969617b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Regenerate

<!-- SRD page 158 -->

Level 7 Transmutation (Bard, Cleric, Druid)\
Casting Time: 1 minute\
Range: Touch\
Components: V, S, M (a prayer wheel)\
Duration: 1 hour\
A creature you touch regains 4d8 + 15 Hit Points.\
For the duration, the target regains 1 Hit Point at\
the start of each of its turns, and any severed body\
parts regrow after 2 minutes.\

Original layout: [[../../srd-5.2.1/pages/158|page 158]].
