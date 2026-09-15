---
splat_schema: 1
splat_id: ee0ba5e2-169f-53ab-a27c-33082a67a6e3
splat_kind: item
splat_key: dnd2024:items/energy_bow
splat_name: Energy Bow
splat_origin:
  key: dnd2024:reference/magic_item/energy_bow
splat_sources:
- title: SRD 5.2.1
  locator: pp. 220
  url: https://www.dndbeyond.com/srd
splat_item:
  category: weapon
  rarity: very_rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/energy_bow
  implementation: dnd2024:item_energy_bow
  parameters:
    weapon_bonus:
      attack: 1
      damage: 1
    ammunition:
      requires_string: false
      generation_trigger: firing_motion
      kind: energy_arrow
      damage_type: force
      replaces_damage_type: piercing
      disappears_on:
      - hit
      - miss
      light:
        bright_radius_ft: 20
        additional_dim_ft: 20
    arrow_of_restraint:
      trigger: ranged_attack_against_creature
      replace_damage: true
      on_hit:
        save:
          ability: strength
          dc: 15
        failure:
          condition: restrained
          duration:
            value: 1
            unit: minute
      escape:
        actor: restrained_target
        action: action
        check:
          ability: strength
          skill: athletics
          dc: 20
        success: end_restraint
    arrow_of_transport:
      action: magic
      range_ft: 60
      target_seen: true
      eligible:
        any_of:
        - kind: creature
          willing: true
          maximum_size: medium
        - kind: object
          worn_or_carried: false
          fits_cube_ft: 5
      destination:
        range_from_user_ft: 10
        seen: true
        unoccupied: true
      effect: teleport
      attack_roll_required: false
    energy_ladder:
      action: magic
      wall_range_ft: 60
      maximum_length_ft: 60
      duration:
        value: 1
        unit: minute
      rungs: glowing_energy_arrows
    attunement:
      required: true
splat_contracts:
- id: dnd2024:item_energy_bow
  inputs:
    user:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    choice:
      required: false
      type: choice
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    locations:
      required: false
      type: location_list
  outputs:
  - effects
  - item_state
  events:
  - item.used
  capabilities:
  - equipment
  - item_resources
  - effects
  - spatial_queries
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: bcbeaaaa03b0c554f1b4e373fdd1cc28f74b4469c0b3c2cda434a8d00356f7fc
  source_path: builtin/srd-5.2.1/magic_item/energy_bow.md
  pages:
  - 220
  engine_gaps: &id001
  - dnd2024:item_energy_bow
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: fd150631955bde5c801cf560fa85df8738c5e9826352989060bf719d252cf76f
splat_clauses:
- key: source_rules
  text: '# Energy Bow Weapon (Longbow or Shortbow), Very Rare (Requires Attunement) You gain a +1 bonus
    to attack rolls and damage rolls made with this magic weapon, which has no string. Each time you pull
    your arm back in a firing motion, a magical arrow made of golden energy appears nocked and ready to
    fire. An arrow produced by this weapon deals Force damage instead of Piercing damage on a hit, and
    it disappears after it hits or misses its target. Until it disappears, the arrow emits Bright Light
    in a 20-foot radius and Dim Light for an additional 20 feet. This weapon has the following additional
    properties. Arrow of Restraint. Whenever you use this weapon to make a ranged attack against a creature,
    you can try to restrain the target instead of dealing damage to it. If the arrow hits, the target
    must succeed on a DC 15 Strength saving throw or have the Restrained condition for 1 minute. As an
    action, a creature Restrained by an arrow can make a DC 20 Strength (Athletics) check to try to break
    the restraint, ending the effect on itself on a successful check. Arrow of Transport. As a Magic action,
    you can fire one energy arrow from this weapon at a target you can see within 60 feet of yourself.
    The target can be either a willing Medium or smaller creature or an object that isn’t being worn or
    carried, provided the object is small enough to fit inside a 5-foot Cube. The arrow teleports the
    target to an unoccupied space you can see within 10 feet of you. Energy Ladder. As a Magic action,
    you can loose a flurry of energy arrows from this weapon at a wall up to 60 feet away from yourself.
    The arrows become glowing rungs that stick out of the wall, forming a magical ladder up to 60 feet
    long on the wall. This ladder lasts for 1 minute before disappearing.'
  sha256: 1a466a463e962f72b1651c940f976caa5aa72a96bd61f2452b3b73c25ac69d1b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Energy Bow

<!-- SRD page 220 -->

Weapon (Longbow or Shortbow), Very Rare (Requires\
Attunement)\
You gain a +1 bonus to attack rolls and damage rolls\
made with this magic weapon, which has no string.\
Each time you pull your arm back in a firing motion,\
a magical arrow made of golden energy appears\
nocked and ready to fire. An arrow produced by\
this weapon deals Force damage instead of Pierc-\
ing damage on a hit, and it disappears after it hits\
or misses its target. Until it disappears, the arrow\
emits Bright Light in a 20-foot radius and Dim Light\
for an additional 20 feet.\
This weapon has the following additional\
properties.\
Arrow of Restraint. Whenever you use this\
weapon to make a ranged attack against a creature,\
you can try to restrain the target instead of deal-\
ing damage to it. If the arrow hits, the target must\
succeed on a DC 15 Strength saving throw or have\
the Restrained condition for 1 minute. As an action,\
a creature Restrained by an arrow can make a DC\
20 Strength (Athletics) check to try to break the\
restraint, ending the effect on itself on a successful\
check.\
Arrow of Transport. As a Magic action, you can\
fire one energy arrow from this weapon at a target\
you can see within 60 feet of yourself. The target\
can be either a willing Medium or smaller creature\
or an object that isn’t being worn or carried, pro-\
vided the object is small enough to fit inside a 5-foot\
Cube. The arrow teleports the target to an unoccu-\
pied space you can see within 10 feet of you.\
Energy Ladder. As a Magic action, you can loose a\
flurry of energy arrows from this weapon at a wall\
up to 60 feet away from yourself. The arrows be-\
come glowing rungs that stick out of the wall, form-\
ing a magical ladder up to 60 feet long on the wall.\
This ladder lasts for 1 minute before disappearing.\

Original layout: [[../../srd-5.2.1/pages/220|page 220]].
