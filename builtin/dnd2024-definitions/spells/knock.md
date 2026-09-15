---
splat_schema: 1
splat_id: 171111ce-3d96-5c9f-99f9-7d39e7f2bb37
splat_kind: spell
splat_key: dnd2024:spells/knock
splat_name: Knock
splat_origin:
  key: dnd2024:reference/spell/knock
splat_sources:
- title: SRD 5.2.1
  locator: pp. 143, 144
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: transmutation
  classes:
  - bard
  - sorcerer
  - wizard
  components:
  - verbal
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    feet: 60
splat_rule:
  mechanic: dnd2024:spells/knock
  implementation: dnd2024:spell_knock
  parameters:
    target:
      kind: access_preventing_object
      count: 1
      visible_to_caster: true
    mundane:
      unlock_locks_maximum: 1
      unstick: true
      unbar: true
    arcane_lock:
      suppress_minutes: 10
      can_open_and_close: true
    sound:
      origin: target
      audible_feet: 300
splat_contracts:
- id: dnd2024:spell_knock
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
  - access_state
  - suppressed_arcane_lock
  - sound
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - inventory
  - time
splat_conversion:
  status: specified
  category: spell
  source_sha256: a1eb22cb343a0e435cd7802696b6c1ac0350eb60941bcc07025e5827278f7649
  source_path: builtin/srd-5.2.1/spell/knock.md
  pages:
  - 143
  - 144
  engine_gaps: &id001
  - dnd2024:spell_knock
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: d26b3c8e4a6702293e2f67adf893d3d06f76c88adb037aeae55719f97494ead6
splat_clauses:
- key: source_rules
  text: '# Knock Level 2 Transmutation (Bard, Sorcerer, Wizard) Casting Time: Action Range: 60 feet Components:
    V Duration: Instantaneous Choose an object that you can see within range. The object can be a door,
    a box, a chest, a set of manacles, a padlock, or another object that contains a mundane or magical
    means that prevents access. A target that is held shut by a mundane lock or that is stuck or barred
    becomes unlocked, unstuck, or unbarred. If the object has multiple locks, only one of them is unlocked.
    If the target is held shut by Arcane Lock, that spell is suppressed for 10 minutes, during which time
    the target can be opened and closed. When you cast the spell, a loud knock, audible up to 300 feet
    away, emanates from the target.'
  sha256: 38a04c830097ff90e909c19d028cb33a06bd6c826b57c239eb4c259c6b94ab2d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Knock

<!-- SRD page 143 -->

Level 2 Transmutation (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V\
Duration: Instantaneous\

<!-- SRD page 144 -->

Choose an object that you can see within range. The\
object can be a door, a box, a chest, a set of mana-\
cles, a padlock, or another object that contains a\
mundane or magical means that prevents access.\
A target that is held shut by a mundane lock or\
that is stuck or barred becomes unlocked, unstuck,\
or unbarred. If the object has multiple locks, only\
one of them is unlocked.\
If the target is held shut by Arcane Lock, that spell\
is suppressed for 10 minutes, during which time the\
target can be opened and closed.\
When you cast the spell, a loud knock, audible up\
to 300 feet away, emanates from the target.\

Original layout: [[../../srd-5.2.1/pages/143|page 143]], [[../../srd-5.2.1/pages/144|page 144]].
