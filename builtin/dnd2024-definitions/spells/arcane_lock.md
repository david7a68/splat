---
splat_schema: 1
splat_id: ebbd596b-a1bb-5673-a0cb-89aab8f3dd7f
splat_kind: spell
splat_key: dnd2024:spells/arcane_lock
splat_name: Arcane Lock
splat_origin:
  key: dnd2024:reference/spell/arcane_lock
splat_sources:
- title: SRD 5.2.1
  locator: pp. 110
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: abjuration
  classes:
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Until dispelled
  casting_time: Action
  ritual: false
  range:
    mode: touch
  material:
    description: gold dust worth 25+ GP, which the spell consumes
    consumed: true
splat_rule:
  mechanic: dnd2024:spells/arcane_lock
  implementation: dnd2024:spell_arcane_lock
  parameters:
    spell_lifetime:
      concentration: false
      until: dispelled_or_source_specific_end
    target:
      kinds:
      - door
      - window
      - gate
      - container
      - hatch
      closed: true
      touch: true
    lock:
      magical: true
      nonmagical_unlock: impossible
    authorized_open_close:
      caster: true
      designated_creatures:
        chooser: caster
        timing: cast
    password:
      optional: true
      chooser: caster
      spoken_within_feet: 5
      unlock_seconds: 60
splat_contracts:
- id: dnd2024:spell_arcane_lock
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    target:
      type: object
      required: true
    authorized_creatures:
      type: participant_list
      required: true
    password:
      required: false
      type: text
  outputs:
  - effects
  - objects
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - world_objects
  - inventory
splat_conversion:
  status: specified
  category: spell
  source_sha256: 479a4f80d3b27c29d62316a14f329e8eeaab0aaf0722e6052f99dea840d1f519
  source_path: builtin/srd-5.2.1/spell/arcane_lock.md
  pages:
  - 110
  engine_gaps: &id001
  - dnd2024:spell_arcane_lock
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: c4c3c7a133878a36595ae9ae158352ab4be6cb7a4d717eb318e8b62b48a31728
splat_clauses:
- key: source_rules
  text: '# Arcane Lock Level 2 Abjuration (Wizard) Casting Time: Action Range: Touch Components: V, S,
    M (gold dust worth 25+ GP, which the spell consumes) Duration: Until dispelled You touch a closed
    door, window, gate, container, or hatch and magically lock it for the duration. This lock can’t be
    unlocked by any nonmagical means. You and any creatures you designate when you cast the spell can
    open and close the object despite the lock. You can also set a password that, when spoken within 5
    feet of the object, unlocks it for 1 minute.'
  sha256: 1077a2bd2dcb0d18aa6ed658e220d0e77d5c64262b5a24ca0c3dbf12df978351
  status: contract_specified
splat_requires_mechanics: *id001
---

# Arcane Lock

<!-- SRD page 110 -->

Level 2 Abjuration (Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (gold dust worth 25+ GP, which\
the spell consumes)\
Duration: Until dispelled\
You touch a closed door, window, gate, container,\
or hatch and magically lock it for the duration. This\
lock can’t be unlocked by any nonmagical means.\
You and any creatures you designate when you cast\
the spell can open and close the object despite the\
lock. You can also set a password that, when spoken\
within 5 feet of the object, unlocks it for 1 minute.\

Original layout: [[../../srd-5.2.1/pages/110|page 110]].
