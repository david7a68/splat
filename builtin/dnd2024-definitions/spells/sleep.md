---
splat_schema: 1
splat_id: aecb4178-b2fb-5b41-9610-a5ad9a398f94
splat_kind: spell
splat_key: dnd2024:spells/sleep
splat_name: Sleep
splat_origin:
  key: dnd2024:reference/spell/sleep
splat_sources:
- title: SRD 5.2.1
  locator: pp. 163
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: enchantment
  classes:
  - bard
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    feet: 60
  material:
    description: a pinch of sand or rose petals
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/sleep
  implementation: dnd2024:spell_sleep
  parameters:
    area:
      shape: sphere
      radius_feet: 5
      origin: chosen_point_within_range
    targets:
      kind: creature
      in_area: true
      chooser: caster
    automatic_save_success_if_any:
    - does_not_sleep
    - immune_to_exhaustion
    initial_save:
      ability: wisdom
      dc:
        binding: save_dc
      failure:
        condition: incapacitated
        until: target_next_turn_end
        then_repeat_save:
          ability: wisdom
          dc:
            binding: save_dc
          failure:
            condition: unconscious
            duration: spell_duration
          success:
            effect: none
      success:
        effect: none
    end_on_target:
    - takes_damage
    - another_creature:
        within_feet: 5
        cost: action
        activity: shake_awake
splat_contracts:
- id: dnd2024:spell_sleep
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
    locations:
      required: false
      type: location_list
    campaign:
      required: false
      type: campaign
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
  - damage.applied
  - movement.completed
  - save.before
  - save.after
  capabilities:
  - spellcasting
  - effects
  - dice
  - spatial_relationships
  - conditions
  - saving_throws
splat_conversion:
  status: specified
  category: spell
  source_sha256: d76f2a9b6e5a083b2298ee0acae823461ced34bd39c59f0cc309468eab4324f2
  source_path: builtin/srd-5.2.1/spell/sleep.md
  pages:
  - 163
  engine_gaps: &id001
  - dnd2024:spell_sleep
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 8241a17ed1b116c0e440e88a2dcad01acb507eaa0f1727dff247f585baaff19f
splat_clauses:
- key: source_rules
  text: '# Sleep Level 1 Enchantment (Bard, Sorcerer, Wizard) Casting Time: Action Range: 60 feet Components:
    V, S, M (a pinch of sand or rose petals) Duration: Concentration, up to 1 minute Each creature of
    your choice in a 5-foot-radius Sphere centered on a point within range must succeed on a Wisdom saving
    throw or have the Incapacitated condition until the end of its next turn, at which point it must repeat
    the save. If the target fails the second save, the target has the Unconscious condition for the duration.
    The spell ends on a target if it takes damage or someone within 5 feet of it takes an action to shake
    it out of the spell’s effect. Creatures that don’t sleep, such as elves, or that have Immunity to
    the Exhaustion condition automatically succeed on saves against this spell.'
  sha256: 3a6b4b37b565fd7816976041ad76275fbb2ebdc41456fafda8158de859356880
  status: contract_specified
splat_requires_mechanics: *id001
---

# Sleep

<!-- SRD page 163 -->

Level 1 Enchantment (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S, M (a pinch of sand or rose petals)\
Duration: Concentration, up to 1 minute\
Each creature of your choice in a 5-foot-radius\
Sphere centered on a point within range must suc-\
ceed on a Wisdom saving throw or have the Inca-\
pacitated condition until the end of its next turn,\
at which point it must repeat the save. If the target\
fails the second save, the target has the Unconscious\
condition for the duration. The spell ends on a tar-\
get if it takes damage or someone within 5 feet of it\
takes an action to shake it out of the spell’s effect.\
Creatures that don’t sleep, such as elves, or that\
have Immunity to the Exhaustion condition auto-\
matically succeed on saves against this spell.\

Original layout: [[../../srd-5.2.1/pages/163|page 163]].
