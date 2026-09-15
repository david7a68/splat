---
splat_schema: 1
splat_id: 4f0f395a-ed54-5d6b-b89b-fd9cc60369b5
splat_kind: spell
splat_key: dnd2024:spells/eyebite
splat_name: Eyebite
splat_origin:
  key: dnd2024:reference/spell/eyebite
splat_sources:
- title: SRD 5.2.1
  locator: pp. 129
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 6
  school: necromancy
  classes:
  - bard
  - sorcerer
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
    mode: self
splat_rule:
  mechanic: dnd2024:spells/eyebite
  implementation: dnd2024:spell_eyebite
  parameters:
    targets:
      kind: creature
      visible: true
      maximum_range_feet: 60
      count_per_activation: 1
      exclude_previous_successful_savers_this_cast: true
    activation:
      initial: spell_cast
      subsequent:
        cost: magic_action
        'on': caster_turn
    save:
      ability: wisdom
      dc:
        binding: save_dc
      failure:
        chosen_effect_duration: spell_duration
      success:
        effect: none
    modes:
      asleep:
        condition: unconscious
        end_if:
        - target_takes_damage
        - another_creature_uses_action_to_shake_awake
      panicked:
        condition: frightened
        fear_source: caster
        each_target_turn:
          action: dash
          movement: away_from_caster
          route:
          - safest
          - shortest
        end_if:
          distance_from_caster_feet_minimum: 60
          cannot_see_caster: true
      sickened:
        condition: poisoned
splat_contracts:
- id: dnd2024:spell_eyebite
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
  source_sha256: b68ad551b918305fdf3cdf9f91c718cd84fbcfc002c90345a4da7f39ee3687c9
  source_path: builtin/srd-5.2.1/spell/eyebite.md
  pages:
  - 129
  engine_gaps: &id001
  - dnd2024:spell_eyebite
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: c5cf150a5610f439f8ae13578a49aa3654a8cbffa553461cfedf280af27ba46b
splat_clauses:
- key: source_rules
  text: '# Eyebite Level 6 Necromancy (Bard, Sorcerer, Warlock, Wizard) Casting Time: Action Range: Self
    Components: V, S Duration: Concentration, up to 1 minute For the duration, your eyes become an inky
    void. One creature of your choice within 60 feet of you that you can see must succeed on a Wisdom
    saving throw or be affected by one of the following effects of your choice for the duration. On each
    of your turns until the spell ends, you can take a Magic action to target another creature but can’t
    target a creature again if it has succeeded on a save against this casting of the spell. Asleep. The
    target has the Unconscious condition. It wakes up if it takes any damage or if another creature takes
    an action to shake it awake. Panicked. The target has the Frightened condition. On each of its turns,
    the Frightened target must take the Dash action and move away from you by the safest and shortest
    route available. If the target moves to a space at least 60 feet away from you where it can’t see
    you, this effect ends. Sickened. The target has the Poisoned condition.'
  sha256: 466b78618da9f392a34fac5731b2236c89df460b4447dc1b8a58c5954d54b563
  status: contract_specified
splat_requires_mechanics: *id001
---

# Eyebite

<!-- SRD page 129 -->

Level 6 Necromancy (Bard, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S\
Duration: Concentration, up to 1 minute\
For the duration, your eyes become an inky void.\
One creature of your choice within 60 feet of you\
that you can see must succeed on a Wisdom saving\
throw or be affected by one of the following effects\
of your choice for the duration.\
On each of your turns until the spell ends, you can\
take a Magic action to target another creature but\
can’t target a creature again if it has succeeded on a\
save against this casting of the spell.\
Asleep. The target has the Unconscious condition.\
It wakes up if it takes any damage or if another\
creature takes an action to shake it awake.\
Panicked. The target has the Frightened con-\
dition. On each of its turns, the Frightened target\
must take the Dash action and move away from you\
by the safest and shortest route available. If the tar-\
get moves to a space at least 60 feet away from you\
where it can’t see you, this effect ends.\
Sickened. The target has the Poisoned condition.\

Original layout: [[../../srd-5.2.1/pages/129|page 129]].
