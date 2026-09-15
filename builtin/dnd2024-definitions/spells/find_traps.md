---
splat_schema: 1
splat_id: 90d3db6a-0ae0-581a-aee9-ca9066d4d441
splat_kind: spell
splat_key: dnd2024:spells/find_traps
splat_name: Find Traps
splat_origin:
  key: dnd2024:reference/spell/find_traps
splat_sources:
- title: SRD 5.2.1
  locator: pp. 131
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: divination
  classes:
  - cleric
  - druid
  - ranger
  components:
  - verbal
  - somatic
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    feet: 120
splat_rule:
  mechanic: dnd2024:spells/find_traps
  implementation: dnd2024:spell_find_traps
  parameters:
    detection:
      within_range: true
      line_of_sight: true
      eligible: object_or_mechanism_created_to_cause_damage_or_danger
      includes_spells:
      - alarm
      - glyph_of_warding
      excludes:
      - natural_floor_weakness
      - unstable_ceiling
      - hidden_sinkhole
      outputs:
      - presence
      - general_danger_nature
      reveal_location: false
splat_contracts:
- id: dnd2024:spell_find_traps
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    locations:
      required: false
      type: location_list
    choices:
      required: false
      type: choice
    resolution:
      required: false
      type: resolution
    campaign:
      required: false
      type: campaign
  outputs:
  - trap_presence
  - danger_nature
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - damage
  - inventory
splat_conversion:
  status: specified
  category: spell
  source_sha256: d0d880180cbd402ccd2baf4db24b014233662bf487bfccc51629bff969b2c191
  source_path: builtin/srd-5.2.1/spell/find_traps.md
  pages:
  - 131
  engine_gaps: &id001
  - dnd2024:spell_find_traps
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: fd90cf13233a426e183c22f87208d38bc88bf07d3fa3960c59a4422bab73e41a
splat_clauses:
- key: source_rules
  text: '# Find Traps Level 2 Divination (Cleric, Druid, Ranger) Casting Time: Action Range: 120 feet
    Components: V, S Duration: Instantaneous You sense any trap within range that is within line of sight.
    A trap, for the purpose of this spell, includes any object or mechanism that was created to cause
    damage or other danger. Thus, the spell would sense the Alarm or Glyph of Warding spell or a mechanical
    pit trap, but it wouldn’t reveal a natural weakness in the floor, an unstable ceiling, or a hidden
    sinkhole. This spell reveals that a trap is present but not its location. You do learn the general
    nature of the danger posed by a trap you sense.'
  sha256: 5d0ba36e31431e8d223e7bc52690d5895362dea422d0cf8af67aadf7f8cdc8da
  status: contract_specified
splat_requires_mechanics: *id001
---

# Find Traps

<!-- SRD page 131 -->

Level 2 Divination (Cleric, Druid, Ranger)\
Casting Time: Action\
Range: 120 feet\
Components: V, S\
Duration: Instantaneous\
You sense any trap within range that is within line\
of sight. A trap, for the purpose of this spell, in-\
cludes any object or mechanism that was created\
to cause damage or other danger. Thus, the spell\
would sense the Alarm or Glyph of Warding spell or a\
mechanical pit trap, but it wouldn’t reveal a natural\
weakness in the floor, an unstable ceiling, or a hid-\
den sinkhole.\
This spell reveals that a trap is present but not its\
location. You do learn the general nature of the dan-\
ger posed by a trap you sense.\

Original layout: [[../../srd-5.2.1/pages/131|page 131]].
