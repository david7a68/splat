---
splat_schema: 1
splat_id: 64232bd1-5e1c-54a7-9975-06561b611acc
splat_kind: spell
splat_key: dnd2024:spells/power_word_stun
splat_name: Power Word Stun
splat_origin:
  key: dnd2024:reference/spell/power_word_stun
splat_sources:
- title: SRD 5.2.1
  locator: pp. 154
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 8
  school: enchantment
  classes:
  - bard
  - sorcerer
  - warlock
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
  mechanic: dnd2024:spells/power_word_stun
  implementation: dnd2024:spell_power_word_stun
  parameters:
    target:
      kind: creature
      count: 1
      visible_to_caster: true
    branches:
    - hp_lte: 150
      condition: stunned
      repeat_save:
        event: turn.end
        ability: constitution
        success: end_stunned
    - hp_gt: 150
      speed: 0
      until: caster.next_turn.start
splat_contracts:
- id: dnd2024:spell_power_word_stun
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
  - stunned
  - speed_effect
  - saves
  events:
  - spell.cast_completed
  - turn.start
  - turn.end
  capabilities:
  - spellcasting
  - effects
  - saving_throws
  - conditions
  - time
splat_conversion:
  status: specified
  category: spell
  source_sha256: dc714ce9646a96ebfa272623bb36c9156177b89d319fbc29090af85b8accb0eb
  source_path: builtin/srd-5.2.1/spell/power_word_stun.md
  pages:
  - 154
  engine_gaps: &id001
  - dnd2024:spell_power_word_stun
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 243d2581470908b63d82a61ab2e06c4c1ec9c9e40b7b67f798832d0b0094c30c
splat_clauses:
- key: source_rules
  text: '# Power Word Stun Level 8 Enchantment (Bard, Sorcerer, Warlock, Wizard) Casting Time: Action
    Range: 60 feet Components: V Duration: Instantaneous You overwhelm the mind of one creature you can
    see within range. If the target has 150 Hit Points or fewer, it has the Stunned condition. Otherwise,
    its Speed is 0 until the start of your next turn. The Stunned target makes a Constitution saving throw
    at the end of each of its turns, ending the condition on itself on a success.'
  sha256: d9cf04094db65dd70668b28eaad7ea057393ba2b5a66f68b7bfd60c6f7270fbe
  status: contract_specified
splat_requires_mechanics: *id001
---

# Power Word Stun

<!-- SRD page 154 -->

Level 8 Enchantment (Bard, Sorcerer, Warlock,\
Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V\
Duration: Instantaneous\
You overwhelm the mind of one creature you can\
see within range. If the target has 150 Hit Points or\
fewer, it has the Stunned condition. Otherwise, its\
Speed is 0 until the start of your next turn.\
The Stunned target makes a Constitution saving\
throw at the end of each of its turns, ending the con-\
dition on itself on a success.\

Original layout: [[../../srd-5.2.1/pages/154|page 154]].
