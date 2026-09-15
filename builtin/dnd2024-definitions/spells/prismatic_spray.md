---
splat_schema: 1
splat_id: 815fe32c-d12a-540a-acde-9d65b2337972
splat_kind: spell
splat_key: dnd2024:spells/prismatic_spray
splat_name: Prismatic Spray
splat_origin:
  key: dnd2024:reference/spell/prismatic_spray
splat_sources:
- title: SRD 5.2.1
  locator: pp. 154, 155
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 7
  school: evocation
  classes:
  - bard
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    mode: self
splat_rule:
  mechanic: dnd2024:spells/prismatic_spray
  implementation: dnd2024:spell_prismatic_spray
  parameters:
    spell_lifetime:
      concentration: false
      instantaneous: true
    area:
      shape: cone
      length_feet: 60
      origin: caster
    targets:
      kind: creature
      selection: all_in_area
    initial_save:
      ability: dexterity
      dc: spell_save_dc
      count_per_target: 1
    ray_roll:
      dice: 1d8
      scope: target
    rays:
    - roll: 1
      color: red
      failure_damage:
        dice: 12d6
        type: fire
      success_damage_scale:
        numerator: 1
        denominator: 2
    - roll: 2
      color: orange
      failure_damage:
        dice: 12d6
        type: acid
      success_damage_scale:
        numerator: 1
        denominator: 2
    - roll: 3
      color: yellow
      failure_damage:
        dice: 12d6
        type: lightning
      success_damage_scale:
        numerator: 1
        denominator: 2
    - roll: 4
      color: green
      failure_damage:
        dice: 12d6
        type: poison
      success_damage_scale:
        numerator: 1
        denominator: 2
    - roll: 5
      color: blue
      failure_damage:
        dice: 12d6
        type: cold
      success_damage_scale:
        numerator: 1
        denominator: 2
    - roll: 6
      color: indigo
      initial_success: unaffected
      initial_failure:
        condition: restrained
      subsequent_save:
        event: turn.end
        subject: target
        ability: constitution
        dc: spell_save_dc
        counters:
        - successes
        - failures
        counters_initial: 0
        consecutive_required: false
        successes_threshold:
          count: 3
          effect: end_restrained
        failures_threshold:
          count: 3
          effect: petrified
          duration: until_freed
        stop_saves_on_threshold: true
      freeing_example:
        key: dnd2024:spells/greater_restoration
    - roll: 7
      color: violet
      initial_success: unaffected
      initial_failure:
        condition: blinded
      subsequent_save:
        event: turn.start
        subject: caster
        occurrence: next
        ability: wisdom
        dc: spell_save_dc
        save_subject: target
        success:
          end_condition: blinded
        failure:
          end_condition: blinded
          teleport:
            plane:
              chooser: gm
              constraint: different_plane
    - roll: 8
      color: special
      ray_count: 2
      roll_each: 1d8
      reroll_values:
      - 8
      apply_both: true
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_prismatic_spray
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    origin:
      type: location
      required: true
    targets:
      type: participant_list
      required: true
    plane_choice:
      required: false
      type: choice
    save_dc:
      type: integer
      required: true
    spatial_facts:
      required: false
      type: resolution
  outputs:
  - effects
  - damage
  - transports
  events:
  - spell.cast_completed
  - turn.end
  - turn.start
  capabilities:
  - spellcasting
  - effects
  - planar_travel
  - areas
  - conditions
  - damage
  - tests
splat_conversion:
  status: specified
  category: spell
  source_sha256: f333bfebbc42fc455131320843292c8c5c37770cefcf704675227eff912ef907
  source_path: builtin/srd-5.2.1/spell/prismatic_spray.md
  pages:
  - 154
  - 155
  engine_gaps: &id001
  - dnd2024:spell_prismatic_spray
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: 2fb79710d04926376da2b1c854c8058fe6572dfb673744e393af350544f6f524
splat_clauses:
- key: source_rules
  text: '# Prismatic Spray Level 7 Evocation (Bard, Sorcerer, Wizard) Casting Time: Action Range: Self
    Components: V, S Duration: Instantaneous Eight rays of light flash from you in a 60-foot Cone. Each
    creature in the Cone makes a Dexterity saving throw. For each target, roll 1d8 to determine which
    color ray affects it, consulting the Prismatic Rays table. Prismatic Rays 1d8 Ray 1 Red. Failed Save:
    12d6 Fire damage. Successful Save: Half as much damage. 2 Orange. Failed Save: 12d6 Acid damage. Successful
    Save: Half as much damage. 3 Yellow. Failed Save: 12d6 Lightning damage. Successful Save: Half as
    much damage. 4 Green. Failed Save: 12d6 Poison damage. Successful Save: Half as much damage. 1d8 Ray
    5 Blue. Failed Save: 12d6 Cold damage. Successful Save: Half as much damage. 6 Indigo. Failed Save:
    The target has the Restrained condition and makes a Constitution saving throw at the end of each of
    its turns. If it successfully saves three times, the condition ends. If it fails three times, it has
    the Petrified condition until it is freed by an effect like the Greater Restoration spell. The successes
    and failures needn’t be consecutive; keep track of both until the target collects three of a kind.
    7 Violet. Failed Save: The target has the Blinded condition and makes a Wisdom saving throw at the
    start of your next turn. On a successful save, the condition ends. On a failed save, the condition
    ends, and the creature teleports to another plane of existence (GM’s choice). 8 Special. The target
    is struck by two rays. Roll twice, rerolling any 8.'
  sha256: 493d582a50e16f7327f862621a46fca299f0284b5e750e4796a15cc55fa2334d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Prismatic Spray

<!-- SRD page 154 -->

Level 7 Evocation (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S\
Duration: Instantaneous\
Eight rays of light flash from you in a 60-foot Cone.\
Each creature in the Cone makes a Dexterity saving\
throw. For each target, roll 1d8 to determine which\
color ray affects it, consulting the Prismatic Rays\
table.\
Prismatic Rays\
1d8 Ray\
1 Red. Failed Save: 12d6 Fire damage. Successful\
Save: Half as much damage.\
2 Orange. Failed Save: 12d6 Acid damage. Successful\
Save: Half as much damage.\
3 Yellow. Failed Save: 12d6 Lightning damage. Suc-\
cessful Save: Half as much damage.\
4 Green. Failed Save: 12d6 Poison damage. Success-\
ful Save: Half as much damage.\

<!-- SRD page 155 -->

1d8 Ray\
5 Blue. Failed Save: 12d6 Cold damage. Successful\
Save: Half as much damage.\
6 Indigo. Failed Save: The target has the Restrained\
condition and makes a Constitution saving throw\
at the end of each of its turns. If it successfully\
saves three times, the condition ends. If it fails\
three times, it has the Petrified condition until it\
is freed by an effect like the Greater Restoration\
spell. The successes and failures needn’t be con-\
secutive; keep track of both until the target col-\
lects three of a kind.\
7 Violet. Failed Save: The target has the Blinded\
condition and makes a Wisdom saving throw at\
the start of your next turn. On a successful save,\
the condition ends. On a failed save, the condi-\
tion ends, and the creature teleports to another\
plane of existence (GM’s choice).\
8 Special. The target is struck by two rays. Roll\
twice, rerolling any 8.\

Original layout: [[../../srd-5.2.1/pages/154|page 154]], [[../../srd-5.2.1/pages/155|page 155]].
