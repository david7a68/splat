---
splat_schema: 1
splat_id: cec90d8e-7a1e-533b-ac52-1cb607f5e548
splat_kind: spell
splat_key: dnd2024:spells/ray_of_enfeeblement
splat_name: Ray of Enfeeblement
splat_origin:
  key: dnd2024:reference/spell/ray_of_enfeeblement
splat_sources:
- title: SRD 5.2.1
  locator: pp. 157
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: necromancy
  classes:
  - warlock
  - wizard
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    feet: 60
splat_rule:
  mechanic: dnd2024:spells/ray_of_enfeeblement
  implementation: dnd2024:spell_ray_of_enfeeblement
  parameters:
    targets:
      kind: creature
      maximum: 1
      within_spell_range: true
    save:
      ability: constitution
      dc:
        binding: save_dc
      failure:
        duration: spell_duration
        disadvantage:
          rolls: d20_tests
          ability: strength
        damage_roll_modifier:
          all_damage_rolls: true
          operation: subtract
          amount:
            dice:
              count: 1
              sides: 8
              add: 0
        repeat_save:
          event: target_turn_end
          success: end_spell
      success:
        disadvantage_next_attack: true
        until: caster_next_turn_start
splat_contracts:
- id: dnd2024:spell_ray_of_enfeeblement
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
    save_dc:
      type: integer
      required: true
    choices:
      required: false
      type: choice
    trigger:
      required: false
      type: event
  outputs:
  - effects
  events:
  - spell.cast_completed
  - turn.end
  - attack.roll
  - attack.hit
  - turn.start
  - save.before
  - save.after
  capabilities:
  - spellcasting
  - effects
  - dice
  - saving_throws
  - roll_modifiers
splat_conversion:
  status: specified
  category: spell
  source_sha256: 32ab189bc6709515d3af30883d33907af473a73f52983fcbc68d8205839d83ea
  source_path: builtin/srd-5.2.1/spell/ray_of_enfeeblement.md
  pages:
  - 157
  engine_gaps: &id001
  - dnd2024:spell_ray_of_enfeeblement
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: bf38c53e19be4506fcf2a782f56a345f06ffc29586751d87e8e1cac7402fd014
splat_clauses:
- key: source_rules
  text: '# Ray of Enfeeblement Level 2 Necromancy (Warlock, Wizard) Casting Time: Action Range: 60 feet
    Components: V, S Duration: Concentration, up to 1 minute A beam of enervating energy shoots from you
    toward a creature within range. The target must make a Constitution saving throw. On a successful
    save, the target has Disadvantage on the next attack roll it makes until the start of your next turn.
    On a failed save, the target has Disadvantage on Strength-based D20 Tests for the duration. During
    that time, it also subtracts 1d8 from all its damage rolls. The target repeats the save at the end
    of each of its turns, ending the spell on a success.'
  sha256: 4bab6372bcc1cc72d2449d10735d92698e3c7eb319e01cd5814aba86b494fa77
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ray of Enfeeblement

<!-- SRD page 157 -->

Level 2 Necromancy (Warlock, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Concentration, up to 1 minute\
A beam of enervating energy shoots from you to-\
ward a creature within range. The target must\
make a Constitution saving throw. On a successful\
save, the target has Disadvantage on the next attack\
roll it makes until the start of your next turn.\
On a failed save, the target has Disadvantage on\
Strength-based D20 Tests for the duration. During\
that time, it also subtracts 1d8 from all its damage\
rolls. The target repeats the save at the end of each\
of its turns, ending the spell on a success.\

Original layout: [[../../srd-5.2.1/pages/157|page 157]].
