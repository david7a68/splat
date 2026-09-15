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
splat_conversion:
  status: partial
  category: spell
  source_sha256: 479a4f80d3b27c29d62316a14f329e8eeaab0aaf0722e6052f99dea840d1f519
  source_path: builtin/srd-5.2.1/spell/arcane_lock.md
  pages:
  - 110
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Arcane Lock Level 2 Abjuration (Wizard) Casting Time: Action Range: Touch Components: V, S,
    M (gold dust worth 25+ GP, which the spell consumes) Duration: Until dispelled You touch a closed
    door, window, gate, container, or hatch and magically lock it for the duration. This lock can’t be
    unlocked by any nonmagical means. You and any creatures you designate when you cast the spell can
    open and close the object despite the lock. You can also set a password that, when spoken within 5
    feet of the object, unlocks it for 1 minute.'
  sha256: 1077a2bd2dcb0d18aa6ed658e220d0e77d5c64262b5a24ca0c3dbf12df978351
  status: needs_semantic_conversion
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
