---
splat_schema: 1
splat_id: 2d64dfd8-a1b9-569d-94fb-30e2b141dfd9
splat_kind: spell
splat_key: dnd2024:spells/blindness_deafness
splat_name: Blindness/Deafness
splat_origin:
  key: dnd2024:reference/spell/blindness_deafness
splat_sources:
- title: SRD 5.2.1
  locator: pp. 113
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: transmutation
  classes:
  - bard
  - cleric
  - sorcerer
  - wizard
  components:
  - verbal
  concentration: false
  duration: 1 minute
  casting_time: Action
  ritual: false
  range:
    feet: 120
splat_rule:
  mechanic: dnd2024:spells/blindness_deafness
  implementation: dnd2024:spell_blindness_deafness
  parameters:
    target:
      kind: creature
      count: 1
      visible_to_caster: true
    condition_choice:
    - blinded
    - deafened
    save:
      ability: constitution
      dc:
        binding: save_dc
      failure:
        condition: chosen
        duration: spell
      success:
        effect: none
    repeat_save:
      event: turn.end
      ability: constitution
      success: end_spell_on_self
    scaling:
      per_slot_above: 2
      additional_targets: 1
splat_contracts:
- id: dnd2024:spell_blindness_deafness
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    save_dc:
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
  - saves
  - condition_effect
  events:
  - spell.cast_completed
  - turn.end
  capabilities:
  - spellcasting
  - effects
  - saving_throws
  - conditions
  - choices
  - time
splat_conversion:
  status: specified
  category: spell
  source_sha256: eba3114003dad33eb690c9cbf6df63d8efc6c060d35d0683f856eb5cfb1c7e61
  source_path: builtin/srd-5.2.1/spell/blindness_deafness.md
  pages:
  - 113
  engine_gaps: &id001
  - dnd2024:spell_blindness_deafness
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 35cf510808f211ae277aa083130239bb89257f0621e5a08622607a08deb2b6e7
splat_clauses:
- key: source_rules
  text: '# Blindness/Deafness Level 2 Transmutation (Bard, Cleric, Sorcerer, Wizard) Casting Time: Action
    Range: 120 feet Components: V Duration: 1 minute One creature that you can see within range must succeed
    on a Constitution saving throw, or it has the Blinded or Deafened condition (your choice) for the
    duration. At the end of each of its turns, the target repeats the save, ending the spell on itself
    on a success. Using a Higher-Level Spell Slot. You can target one additional creature for each spell
    slot level above 2.'
  sha256: 32c54b6f6086a83b278cea976dacb04a6e0ef14e821522de402dd86137108eef
  status: contract_specified
splat_requires_mechanics: *id001
---

# Blindness/Deafness

<!-- SRD page 113 -->

Level 2 Transmutation (Bard, Cleric, Sorcerer, Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: V\
Duration: 1 minute\
One creature that you can see within range must\
succeed on a Constitution saving throw, or it has\
the Blinded or Deafened condition (your choice)\
for the duration. At the end of each of its turns, the\
target repeats the save, ending the spell on itself on\
a success.\
Using a Higher-Level Spell Slot. You can target\
one additional creature for each spell slot level\
above 2.\

Original layout: [[../../srd-5.2.1/pages/113|page 113]].
