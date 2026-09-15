---
splat_schema: 1
splat_id: b236f1a7-8b7d-57c6-8fb5-d5df6bfb0218
splat_kind: spell
splat_key: dnd2024:spells/death_ward
splat_name: Death Ward
splat_origin:
  key: dnd2024:reference/spell/death_ward
splat_sources:
- title: SRD 5.2.1
  locator: pp. 122
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 4
  school: abjuration
  classes:
  - cleric
  - paladin
  components:
  - verbal
  - somatic
  concentration: false
  duration: 8 hours
  casting_time: Action
  ritual: false
  range:
    mode: touch
splat_rule:
  mechanic: dnd2024:spells/death_ward
  implementation: dnd2024:spell_death_ward
  parameters:
    targeting:
      kind: creature
      touch: true
      count: 1
    replacements:
    - trigger: first_hp_drop_to_zero
      set_hp: 1
      end_spell: true
    - trigger: instant_death_without_damage
      negate_for_target: true
      end_spell: true
    duration: spell_duration
    spell_lifetime:
      concentration: false
      maximum:
        value: 8
        unit: hour
splat_contracts:
- id: dnd2024:spell_death_ward
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
  - damage.applied
  - health.changed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - damage
splat_conversion:
  status: specified
  category: spell
  source_sha256: 6107de8f8477be99c2eb096a73b80ac43f5addce93964b01f395e99ce44a11cd
  source_path: builtin/srd-5.2.1/spell/death_ward.md
  pages:
  - 122
  engine_gaps: &id001
  - dnd2024:spell_death_ward
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 4d7485a20d042b55ba18eb1a192525a7cb6ad87f16c52d80d59314995937b221
splat_clauses:
- key: source_rules
  text: '# Death Ward Level 4 Abjuration (Cleric, Paladin) Casting Time: Action Range: Touch Components:
    V, S Duration: 8 hours You touch a creature and grant it a measure of protection from death. The first
    time the target would drop to 0 Hit Points before the spell ends, the target instead drops to 1 Hit
    Point, and the spell ends. If the spell is still in effect when the target is subjected to an effect
    that would kill it instantly without dealing damage, that effect is negated against the target, and
    the spell ends.'
  sha256: 68320f663c57dabe516aa48e6d0f4ae7959cda5f8eb506659b67c5f7dcc9c35d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Death Ward

<!-- SRD page 122 -->

Level 4 Abjuration (Cleric, Paladin)\
Casting Time: Action\
Range: Touch\
Components: V, S\
Duration: 8 hours\
You touch a creature and grant it a measure of pro-\
tection from death. The first time the target would\
drop to 0 Hit Points before the spell ends, the target\
instead drops to 1 Hit Point, and the spell ends.\
If the spell is still in effect when the target is sub-\
jected to an effect that would kill it instantly with-\
out dealing damage, that effect is negated against\
the target, and the spell ends.\

Original layout: [[../../srd-5.2.1/pages/122|page 122]].
