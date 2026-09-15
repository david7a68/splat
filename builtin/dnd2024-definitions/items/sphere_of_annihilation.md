---
splat_schema: 1
splat_id: 389d6bd9-cf4b-5557-82f9-27aa039b1fa7
splat_kind: item
splat_key: dnd2024:items/sphere_of_annihilation
splat_name: Sphere of Annihilation
splat_origin:
  key: dnd2024:reference/magic_item/sphere_of_annihilation
splat_sources:
- title: SRD 5.2.1
  locator: pp. 244, 245
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: legendary
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/sphere_of_annihilation
  implementation: dnd2024:item_sphere_of_annihilation
  parameters:
    diameter_ft: 2
    hovering: true
    default_motion: stationary
    contact:
      wholly_engulfed: matter_obliterated
      partly_touching:
        damage:
          amount:
            roll:
              count: 8
              sides: 10
          type: force
      artifact_exception: unharmed_unless_artifact_explicitly_susceptible
    control:
      action: magic
      maximum_range_ft: 60
      check:
        ability: intelligence
        skill: arcana
        dc: 25
        success:
          control_until: user.next_turn_start
          previous_controller_loses_control: true
        failure:
          move_ft: 10
          direction: straight_toward_user
    controlled_movement:
      action: bonus_action
      direction_choice: one_direction
      maximum_distance_ft:
        max:
        - 5
        - multiply:
          - 5
          - user.intelligence_modifier
    interaction:
      trigger:
      - planar_portal_contact
      - extradimensional_space_contact
      die: 100
      rows:
      - range:
        - 1
        - 50
        destroy_sphere: true
      - range:
        - 51
        - 85
        sphere: passes_through_or_enters
      - range:
        - 86
        - 100
        transport:
        - sphere
        - creatures
        - objects
        radius_ft: 180
        destination_plane: random
    on_any_sphere_movement_into_creature_space:
      save:
        ability: dexterity
        dc: 19
        failure:
          damage:
            amount:
              roll:
                count: 8
                sides: 10
            type: force
      zero_hp:
        obliterate_body: true
        leave_possessions: true
        physical_remains: false
    attunement_required: false
splat_contracts:
- id: dnd2024:item_sphere_of_annihilation
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    destination:
      required: false
      type: location
    selection:
      required: false
      type: choice
    context:
      required: false
      type: resolution
  outputs:
  - effects
  - item_state_changes
  events:
  - item.activation_requested
  - matter.contact
  - movement.completed
  - turn.started
  - portal.contact
  - extradimensional_space.contact
  capabilities:
  - item_effects
  - dice
  - spatial_targeting
  - planar_travel
  - damage
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: d692f6188c7fd4a9434746b7e771853add263e57539de644ca81cfe9d8e71399
  source_path: builtin/srd-5.2.1/magic_item/sphere_of_annihilation.md
  pages:
  - 244
  - 245
  engine_gaps: &id001
  - dnd2024:item_sphere_of_annihilation
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 348cfb27ee492a6bfdd4f7771ea09b6a04e12527b95243504de7ef4d02a12b91
splat_clauses:
- key: source_rules
  text: '# Sphere of Annihilation Wondrous Item, Legendary This 2-foot-diameter black sphere is a hole
    in the multiverse, hovering in space and stabilized by a magical field surrounding it. The sphere
    obliterates all matter it passes through and all matter that passes through it. Artifacts are the
    exception. Unless an Artifact is susceptible to damage from a Sphere of Annihilation, it passes through
    the sphere unscathed. Anything else that touches the sphere but isn’t wholly engulfed and obliterated
    by it takes 8d10 Force damage. Controlling the Sphere. A Sphere of Annihilation is stationary until
    someone takes control of it. If you are within 60 feet of a sphere, you can take a Magic action to
    make a DC 25 Intelligence (Arcana) check. On a successful check, you control the sphere until the
    start of your next turn, and if it was under another creature’s control, that creature loses control
    of the sphere. On a failed check, the sphere moves 10 feet toward you in a straight line. While in
    control of the sphere, you can take a Bonus Action to cause it to move in one direction of your choice,
    up to a number of feet equal to 5 times your Intelligence modifier (minimum 5 feet). Any creature
    whose space the sphere enters must succeed on a DC 19 Dexterity saving throw or be touched by it,
    taking 8d10 Force damage. A creature reduced to 0 Hit Points by this damage is obliterated, leaving
    its possessions behind but no other physical remains. Sphere Interactions. If the sphere comes into
    contact with a planar portal (such as that created by the Gate spell) or an extradimensional space
    (such as that within a Portable Hole), the GM determines randomly what happens using the following
    table. 1d100 Result 01–50 The sphere is destroyed. 51–85 The sphere moves through the portal or into
    the extradimensional space. 86–00 A spatial rift sends the sphere and each creature and object within
    180 feet of the sphere to a random plane of existence.'
  sha256: 2759be1f98a85fc6beeb0419936fcd50ed0b0f9644c636eabc1057f03fbf73dc
  status: contract_specified
splat_requires_mechanics: *id001
---

# Sphere of Annihilation

<!-- SRD page 244 -->

Wondrous Item, Legendary\
This 2-foot-diameter black sphere is a hole in the\
multiverse, hovering in space and stabilized by a\
magical field surrounding it.\
The sphere obliterates all matter it passes\
through and all matter that passes through it. Ar-\
tifacts are the exception. Unless an Artifact is sus-\
ceptible to damage from a Sphere of Annihilation, it\
passes through the sphere unscathed. Anything else\
that touches the sphere but isn’t wholly engulfed\
and obliterated by it takes 8d10 Force damage.\
Controlling the Sphere. A Sphere of Annihilation is\
stationary until someone takes control of it. If you\
are within 60 feet of a sphere, you can take a Magic\
action to make a DC 25 Intelligence (Arcana) check.\
On a successful check, you control the sphere until\
the start of your next turn, and if it was under an-\
other creature’s control, that creature loses control\
of the sphere. On a failed check, the sphere moves\
10 feet toward you in a straight line.\
While in control of the sphere, you can take a\
Bonus Action to cause it to move in one direction\
of your choice, up to a number of feet equal to\
5 times your Intelligence modifier (minimum 5\
feet). Any creature whose space the sphere enters\
must succeed on a DC 19 Dexterity saving throw\
or be touched by it, taking 8d10 Force damage. A\
creature reduced to 0 Hit Points by this damage is\
obliterated, leaving its possessions behind but no\
other physical remains.\
Sphere Interactions. If the sphere comes into\
contact with a planar portal (such as that created by\
the Gate spell) or an extradimensional space (such\
as that within a Portable Hole), the GM determines\
randomly what happens using the following table.\

<!-- SRD page 245 -->

1d100 Result\
01–50 The sphere is destroyed.\
51–85 The sphere moves through the portal or into\
the extradimensional space.\
86–00 A spatial rift sends the sphere and each crea-\
ture and object within 180 feet of the sphere\
to a random plane of existence.\

Original layout: [[../../srd-5.2.1/pages/244|page 244]], [[../../srd-5.2.1/pages/245|page 245]].
