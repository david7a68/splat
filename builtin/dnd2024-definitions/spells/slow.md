---
splat_schema: 1
splat_id: 1ce5ee88-00e0-5c6b-ac9c-36b851c8227b
splat_kind: spell
splat_key: dnd2024:spells/slow
splat_name: Slow
splat_origin:
  key: dnd2024:reference/spell/slow
splat_sources:
- title: SRD 5.2.1
  locator: pp. 163
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: transmutation
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
    feet: 120
  material:
    description: a drop of molasses
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/slow
  implementation: dnd2024:spell_slow
  parameters:
    area:
      shape: cube
      side_feet: 40
    target:
      kind: creature
      count:
        maximum: 6
      visible_to_caster: false
    save:
      ability: wisdom
      dc:
        binding: save_dc
      failure:
        affected: true
      success:
        effect: none
    affected:
      speed_fraction:
        numerator: 1
        denominator: 2
      ac_delta: -2
      dexterity_save_delta: -2
      reactions: false
      on_turn_action_choice:
      - action
      - bonus_action
      both_action_and_bonus_action: false
      attack_action_maximum_attacks: 1
      somatic_spell_failure_percent: 25
    repeat_save:
      event: turn.end
      ability: wisdom
      success: end_spell_on_self
splat_contracts:
- id: dnd2024:spell_slow
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
    spell_attack_modifier:
      required: false
      type: integer
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
  - slow_effect
  - spell_failure_result
  events:
  - spell.cast_completed
  - turn.end
  capabilities:
  - spellcasting
  - effects
  - saving_throws
  - attacks
  - spatial_areas
  - choices
  - time
splat_conversion:
  status: specified
  category: spell
  source_sha256: 250497db2f0dfbf1e61fb69af0605c082bea6012b8a8e2b1af5120bc9e7ad8ce
  source_path: builtin/srd-5.2.1/spell/slow.md
  pages:
  - 163
  engine_gaps: &id001
  - dnd2024:spell_slow
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 4bfb7beb55db706b8a43266ccfb10b933d3d5d294f5e2ef9729554cb3cdb1f65
splat_clauses:
- key: source_rules
  text: '# Slow Level 3 Transmutation (Bard, Sorcerer, Wizard) Casting Time: Action Range: 120 feet Components:
    V, S, M (a drop of molasses) Duration: Concentration, up to 1 minute You alter time around up to six
    creatures of your choice in a 40-foot Cube within range. Each target must succeed on a Wisdom saving
    throw or be affected by this spell for the duration. An affected target’s Speed is halved, it takes
    a -2 penalty to AC and Dexterity saving throws, and it can’t take Reactions. On its turns, it can
    take either an action or a Bonus Action, not both, and it can make only one attack if it takes the
    Attack action. If it casts a spell with a Somatic component, there is a 25 percent chance the spell
    fails as a result of the target making the spell’s gestures too slowly. An affected target repeats
    the save at the end of each of its turns, ending the spell on itself on a success.'
  sha256: baaac85f48c7bc958c4028e70056052ebeba5f424e5b2122083c127177d39be9
  status: contract_specified
splat_requires_mechanics: *id001
---

# Slow

<!-- SRD page 163 -->

Level 3 Transmutation (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: V, S, M (a drop of molasses)\
Duration: Concentration, up to 1 minute\
You alter time around up to six creatures of your\
choice in a 40-foot Cube within range. Each target\
must succeed on a Wisdom saving throw or be af-\
fected by this spell for the duration.\
An affected target’s Speed is halved, it takes a −2\
penalty to AC and Dexterity saving throws, and it\
can’t take Reactions. On its turns, it can take either\
an action or a Bonus Action, not both, and it can\
make only one attack if it takes the Attack action. If\
it casts a spell with a Somatic component, there is\
a 25 percent chance the spell fails as a result of the\
target making the spell’s gestures too slowly.\
An affected target repeats the save at the end\
of each of its turns, ending the spell on itself on a\
success.\

Original layout: [[../../srd-5.2.1/pages/163|page 163]].
