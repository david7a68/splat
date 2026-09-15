---
splat_schema: 1
splat_id: 1599e01f-a7f5-552c-8392-e402e8d98cf0
splat_kind: spell
splat_key: dnd2024:spells/earthquake
splat_name: Earthquake
splat_origin:
  key: dnd2024:reference/spell/earthquake
splat_sources:
- title: SRD 5.2.1
  locator: pp. 127
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 8
  school: transmutation
  classes:
  - cleric
  - druid
  - sorcerer
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    feet: 500
  material:
    description: a fractured rock
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/earthquake
  implementation: dnd2024:spell_earthquake
  parameters:
    area:
      shape: circle
      radius_feet: 100
      ground: true
      origin_visible: true
      difficult_terrain: true
    creature_effect:
      triggers:
      - spell.cast_completed
      - caster.turn.end
      target: creatures_on_ground_in_area
      save:
        ability: dexterity
        dc:
          binding: save_dc
        failure:
          condition: prone
          break_concentration: true
        success:
          effect: none
    optional_effects:
      fissures:
        event: caster.turn.end
        turn: cast_turn
        count:
          dice:
            count: 1
            sides: 6
            add: 0
        locations_chooser: caster
        under_structures: false
        depth_feet:
          op: multiply
          args:
          - dice:
              count: 1
              sides: 10
              add: 0
          - 10
        width_feet: 10
        extent: area_edge_to_edge
        overlapping_creature_save:
          ability: dexterity
          dc:
            binding: save_dc
          failure:
            fall_into_fissure: true
          success:
            move_with_fissure_edge: true
      structures:
        triggers:
        - spell.cast_completed
        - caster.turn.end
        eligible: ground_contact_in_area
        damage:
          amount: 50
          type: bludgeoning
        collapse_at_hp: 0
        collapse_creature_radius:
          structure_height_fraction:
            numerator: 1
            denominator: 2
        save:
          ability: dexterity
          dc:
            binding: save_dc
          failure:
            damage:
              amount:
                dice:
                  count: 12
                  sides: 6
                  add: 0
              type: bludgeoning
            condition: prone
            buried: true
          success:
            damage_fraction:
              numerator: 1
              denominator: 2
            buried: false
            prone: false
        escape:
          action: action
          ability: strength
          skill: athletics
          dc: 20
splat_contracts:
- id: dnd2024:spell_earthquake
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
  - area_effect
  - saves
  - prone
  - concentration_ended
  - fissures
  - structure_damage
  - collapse
  - burial
  events:
  - spell.cast_completed
  - turn.end
  capabilities:
  - spellcasting
  - effects
  - damage
  - saving_throws
  - conditions
  - spatial_areas
  - movement
  - choices
  - time
  - checks
splat_conversion:
  status: specified
  category: spell
  source_sha256: e326463a35dddacbee4bf78b8ce8cecffa98619a5a6fb86fdefb0dbf8b7c6c8c
  source_path: builtin/srd-5.2.1/spell/earthquake.md
  pages:
  - 127
  engine_gaps: &id001
  - dnd2024:spell_earthquake
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 1b000bf845a27d9307b63d4361e3a89bb6cafe9937fbdff08e46cb3c295926df
splat_clauses:
- key: source_rules
  text: '# Earthquake Level 8 Transmutation (Cleric, Druid, Sorcerer) Casting Time: Action Range: 500
    feet Components: V, S, M (a fractured rock) Duration: Concentration, up to 1 minute Choose a point
    on the ground that you can see within range. For the duration, an intense tremor rips through the
    ground in a 100-foot-radius circle centered on that point. The ground there is Difficult Terrain.
    When you cast this spell and at the end of each of your turns for the duration, each creature on the
    ground in the area makes a Dexterity saving throw. On a failed save, a creature has the Prone condition,
    and its Concentration is broken. You can also cause the effects below. Fissures. A total of 1d6 fissures
    open in the spell’s area at the end of the turn you cast it. You choose the fissures’ locations, which
    can’t be under structures. Each fissure is 1d10 × 10 feet deep and 10 feet wide, and it extends from
    one edge of the spell’s area to another edge. A creature in the same space as a fissure must succeed
    on a Dexterity saving throw or fall in. A creature that successfully saves moves with the fissure’s
    edge as it opens. Structures. The tremor deals 50 Bludgeoning damage to any structure in contact with
    the ground in the area when you cast the spell and at the end of each of your turns until the spell
    ends. If a structure drops to 0 Hit Points, it collapses. A creature within a distance from a collapsing
    structure equal to half the structure’s height makes a Dexterity saving throw. On a failed save, the
    creature takes 12d6 Bludgeoning damage, has the Prone condition, and is buried in the rubble, requiring
    a DC 20 Strength (Athletics) check as an action to escape. On a successful save, the creature takes
    half as much damage only.'
  sha256: d8e1f545d43329c81a20a88c20f01d8a8cf6b929a469378b440b956607b4cdac
  status: contract_specified
splat_requires_mechanics: *id001
---

# Earthquake

<!-- SRD page 127 -->

Level 8 Transmutation (Cleric, Druid, Sorcerer)\
Casting Time: Action\
Range: 500 feet\
Components: V, S, M (a fractured rock)\
Duration: Concentration, up to 1 minute\
Choose a point on the ground that you can see\
within range. For the duration, an intense tremor\
rips through the ground in a 100-foot-radius circle\
centered on that point. The ground there is Difficult\
Terrain.\
When you cast this spell and at the end of each of\
your turns for the duration, each creature on the\
ground in the area makes a Dexterity saving throw.\
On a failed save, a creature has the Prone condition,\
and its Concentration is broken.\
You can also cause the effects below.\
Fissures. A total of 1d6 fissures open in the spell’s\
area at the end of the turn you cast it. You choose\
the fissures’ locations, which can’t be under struc-\
tures. Each fissure is 1d10 × 10 feet deep and 10\
feet wide, and it extends from one edge of the spell’s\
area to another edge. A creature in the same space\
as a fissure must succeed on a Dexterity saving\
throw or fall in. A creature that successfully saves\
moves with the fissure’s edge as it opens.\
Structures. The tremor deals 50 Bludgeoning\
damage to any structure in contact with the ground\
in the area when you cast the spell and at the end of\
each of your turns until the spell ends. If a structure\
drops to 0 Hit Points, it collapses.\
A creature within a distance from a collapsing\
structure equal to half the structure’s height makes\
a Dexterity saving throw. On a failed save, the crea-\
ture takes 12d6 Bludgeoning damage, has the Prone\
condition, and is buried in the rubble, requiring a\
DC 20 Strength (Athletics) check as an action to es-\
cape. On a successful save, the creature takes half as\
much damage only.\

Original layout: [[../../srd-5.2.1/pages/127|page 127]].
