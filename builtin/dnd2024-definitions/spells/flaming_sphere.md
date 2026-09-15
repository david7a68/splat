---
splat_schema: 1
splat_id: d39e9d55-9f59-51d3-af63-660d2d0a482f
splat_kind: spell
splat_key: dnd2024:spells/flaming_sphere
splat_name: Flaming Sphere
splat_origin:
  key: dnd2024:reference/spell/flaming_sphere
splat_sources:
- title: SRD 5.2.1
  locator: pp. 132
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: conjuration
  classes:
  - druid
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
    description: a ball of wax
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/flaming_sphere
  implementation: dnd2024:spell_flaming_sphere
  parameters:
    sphere:
      diameter_feet: 5
      initial_space: unoccupied_ground
      bright_radius_feet: 20
      additional_dim_feet: 20
    damage_trigger:
      event: turn.end
      distance_feet: 5
      save:
        ability: dexterity
        dc:
          binding: save_dc
        failure:
          damage:
            amount:
              dice:
                count: 2
                sides: 6
                add: 0
            type: fire
        success:
          damage_fraction:
            numerator: 1
            denominator: 2
    move:
      action: bonus_action
      maximum_feet: 30
      along_ground: true
      barrier_max_height_feet: 5
      pit_max_width_feet: 10
      creature_space_entry:
        repeat_damage_save: true
        stop_movement_for_turn: true
    objects:
      on_touch: true
      flammable_unworn_uncarried: burning
    scaling:
      per_slot_above: 2
      damage_dice: 1
      sides: 6
splat_contracts:
- id: dnd2024:spell_flaming_sphere
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
  - sphere_effect
  - saves
  - damage
  - burning_objects
  events:
  - spell.cast_completed
  - turn.end
  capabilities:
  - spellcasting
  - effects
  - damage
  - saving_throws
  - spatial_areas
  - movement
  - inventory
  - time
splat_conversion:
  status: specified
  category: spell
  source_sha256: ae450aee5c3bd8816a27346914220636ffa45b1b997ec30e4fb5fdec671e9c60
  source_path: builtin/srd-5.2.1/spell/flaming_sphere.md
  pages:
  - 132
  engine_gaps: &id001
  - dnd2024:spell_flaming_sphere
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 570728523e184e72f914f94e229a248d8f2d712e3a8c868a9173952fd74ddfaa
splat_clauses:
- key: source_rules
  text: '# Flaming Sphere Level 2 Conjuration (Druid, Sorcerer, Wizard) Casting Time: Action Range: 60
    feet Components: V, S, M (a ball of wax) Duration: Concentration, up to 1 minute You create a 5-foot-diameter
    sphere of fire in an unoccupied space on the ground within range. It lasts for the duration. Any creature
    that ends its turn within 5 feet of the sphere makes a Dexterity saving throw, taking 2d6 Fire damage
    on a failed save or half as much damage on a successful one. As a Bonus Action, you can move the sphere
    up to 30 feet, rolling it along the ground. If you move the sphere into a creature’s space, that creature
    makes the save against the sphere, and the sphere stops moving for the turn. When you move the sphere,
    you can direct it over barriers up to 5 feet tall and jump it across pits up to 10 feet wide. Flammable
    objects that aren’t being worn or carried start burning if touched by the sphere, and it sheds Bright
    Light in a 20-foot radius and Dim Light for an additional 20 feet. Using a Higher-Level Spell Slot.
    The damage increases by 1d6 for each spell slot level above 2.'
  sha256: 55a856bc4ca5b32208586e188158484a48e97e83ece3acfaf8f16b1dd68d3b68
  status: contract_specified
splat_requires_mechanics: *id001
---

# Flaming Sphere

<!-- SRD page 132 -->

Level 2 Conjuration (Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S, M (a ball of wax)\
Duration: Concentration, up to 1 minute\
You create a 5-foot-diameter sphere of fire in an un-\
occupied space on the ground within range. It lasts\
for the duration. Any creature that ends its turn\
within 5 feet of the sphere makes a Dexterity saving\
throw, taking 2d6 Fire damage on a failed save or\
half as much damage on a successful one.\
As a Bonus Action, you can move the sphere up to\
30 feet, rolling it along the ground. If you move the\
sphere into a creature’s space, that creature makes\
the save against the sphere, and the sphere stops\
moving for the turn.\
When you move the sphere, you can direct it over\
barriers up to 5 feet tall and jump it across pits up\
to 10 feet wide. Flammable objects that aren’t be-\
ing worn or carried start burning if touched by the\
sphere, and it sheds Bright Light in a 20-foot radius\
and Dim Light for an additional 20 feet.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d6 for each spell slot level above 2.\

Original layout: [[../../srd-5.2.1/pages/132|page 132]].
