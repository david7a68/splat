---
splat_schema: 1
splat_id: a5e28636-7477-5376-a7d6-7a91d1928c65
splat_kind: spell
splat_key: dnd2024:spells/revivify
splat_name: Revivify
splat_origin:
  key: dnd2024:reference/spell/revivify
splat_sources:
- title: SRD 5.2.1
  locator: pp. 159
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: necromancy
  classes:
  - cleric
  - druid
  - paladin
  - ranger
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    mode: touch
  material:
    description: a diamond worth 300+ GP, which the spell consumes
    consumed: true
splat_rule:
  mechanic: dnd2024:spells/revivify
  implementation: dnd2024:spell_revivify
  parameters:
    target:
      kind: creature
      count: 1
      visible_to_caster: false
      touch: true
      dead_within_minutes: 1
      death_by_old_age: false
    revive:
      hp: 1
      restore_missing_parts: false
splat_contracts:
- id: dnd2024:spell_revivify
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
  - revived_creature
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - healing
  - time
splat_conversion:
  status: specified
  category: spell
  source_sha256: 5e4d2372d37c0675e0ec438f19ec47802c27d9011cf3930c5f2b4d4ca3c07479
  source_path: builtin/srd-5.2.1/spell/revivify.md
  pages:
  - 159
  engine_gaps: &id001
  - dnd2024:spell_revivify
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 8b61ab498e9f741bb1cc5df217888f42fdb189e01a9997b0dd70496c94877560
splat_clauses:
- key: source_rules
  text: '# Revivify Level 3 Necromancy (Cleric, Druid, Paladin, Ranger) Casting Time: Action Range: Touch
    Components: V, S, M (a diamond worth 300+ GP, which the spell consumes) Duration: Instantaneous You
    touch a creature that has died within the last minute. That creature revives with 1 Hit Point. This
    spell can’t revive a creature that has died of old age, nor does it restore any missing body parts.'
  sha256: 44f297ca5a8a97ce4a3724ade49af367dff03f2b64b4b963864aec6ff9a0b0f9
  status: contract_specified
splat_requires_mechanics: *id001
---

# Revivify

<!-- SRD page 159 -->

Level 3 Necromancy (Cleric, Druid, Paladin, Ranger)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (a diamond worth 300+ GP, which\
the spell consumes)\
Duration: Instantaneous\
You touch a creature that has died within the last\
minute. That creature revives with 1 Hit Point. This\
spell can’t revive a creature that has died of old age,\
nor does it restore any missing body parts.\

Original layout: [[../../srd-5.2.1/pages/159|page 159]].
