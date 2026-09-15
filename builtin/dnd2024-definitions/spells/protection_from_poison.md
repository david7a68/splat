---
splat_schema: 1
splat_id: d2717406-1b0f-509c-a13e-75424455d68c
splat_kind: spell
splat_key: dnd2024:spells/protection_from_poison
splat_name: Protection from Poison
splat_origin:
  key: dnd2024:reference/spell/protection_from_poison
splat_sources:
- title: SRD 5.2.1
  locator: pp. 157
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: abjuration
  classes:
  - cleric
  - druid
  - paladin
  - ranger
  components:
  - verbal
  - somatic
  concentration: false
  duration: 1 hour
  casting_time: Action
  ritual: false
  range:
    mode: touch
splat_rule:
  mechanic: dnd2024:spells/protection_from_poison
  implementation: dnd2024:spell_protection_from_poison
  parameters:
    spell_lifetime:
      concentration: false
      maximum_seconds: 3600
    target:
      kind: creature
      touch: true
    immediate:
      end_condition: poisoned
    benefits:
      save_advantage:
        condition: poisoned
        purposes:
        - avoid
        - end
      damage_resistance: poison
splat_contracts:
- id: dnd2024:spell_protection_from_poison
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
  - conditions
splat_conversion:
  status: specified
  category: spell
  source_sha256: 64b6303facc44d95660992a176d889fed23b97052198b614abe049814be58d5c
  source_path: builtin/srd-5.2.1/spell/protection_from_poison.md
  pages:
  - 157
  engine_gaps: &id001
  - dnd2024:spell_protection_from_poison
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: e413e7b7831654c5a9f3427822a7d68bb70e22862d3cb8f98b23a1d20c0fac89
splat_clauses:
- key: source_rules
  text: '# Protection from Poison Level 2 Abjuration (Cleric, Druid, Paladin, Ranger) Casting Time: Action
    Range: Touch Components: V, S Duration: 1 hour You touch a creature and end the Poisoned condition
    on it. For the duration, the target has Advantage on saving throws to avoid or end the Poisoned condition,
    and it has Resistance to Poison damage.'
  sha256: 04736a67738105169800681cfaa33a0384d796b38ceee8aff9fb993b32b477d2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Protection from Poison

<!-- SRD page 157 -->

Level 2 Abjuration (Cleric, Druid, Paladin, Ranger)\
Casting Time: Action\
Range: Touch\
Components: V, S\
Duration: 1 hour\
You touch a creature and end the Poisoned condi-\
tion on it. For the duration, the target has Advan-\
tage on saving throws to avoid or end the Poisoned\
condition, and it has Resistance to Poison damage.\

Original layout: [[../../srd-5.2.1/pages/157|page 157]].
