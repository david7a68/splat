---
splat_schema: 1
splat_id: f9868b1e-e538-5f44-b7a1-900adb0fca9c
splat_kind: item
splat_key: dnd2024:items/ring_of_shooting_stars
splat_name: Ring of Shooting Stars
splat_origin:
  key: dnd2024:reference/magic_item/ring_of_shooting_stars
splat_sources:
- title: SRD 5.2.1
  locator: pp. 238, 239
  url: https://www.dndbeyond.com/srd
splat_item:
  category: ring
  rarity: very_rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/ring_of_shooting_stars
  implementation: dnd2024:item_ring_of_shooting_stars
  parameters:
    spell_grants:
    - spell:
        key: dnd2024:spells/dancing_lights
      charge_cost: 0
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
    - spell:
        key: dnd2024:spells/light
      charge_cost: 0
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
    - spell:
        key: dnd2024:spells/faerie_fire
      charge_cost: 1
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
    charges:
      maximum: 6
      initial: 6
      recovery:
        event: dawn
        amount:
          expression: 1d6
          scope: use
        cap: maximum
    lightning_spheres:
      action: magic
      charge_cost: 2
      count:
        minimum: 1
        maximum: 4
        chooser: user
      sphere_diameter_ft: 3
      initial_location:
        unoccupied: true
        visible_to_user: true
        range_ft: 120
      duration:
        amount: 1
        unit: minute
      concentration: true
      dim_light_radius_ft: 30
      move:
        action: bonus_action
        each_sphere_maximum_ft: 30
        maximum_distance_from_user_ft: 120
      discharge:
        trigger: first_within_five_feet_of_eligible_creature
        distance_ft: 5
        exclude_user: true
        exclude_total_cover: true
        destroy_sphere: true
        save:
          ability: dexterity
          dc: 15
          failure:
            damage:
              type: lightning
              roll_by_initial_sphere_count:
                '1':
                  expression: 4d12
                  scope: use
                '2':
                  expression: 5d4
                  scope: use
                '3':
                  expression: 2d6
                  scope: use
                '4':
                  expression: 2d4
                  scope: use
          success:
            damage_scale:
              numerator: 1
              denominator: 2
    shooting_stars:
      action: magic
      charge_cost:
        minimum: 1
        maximum: 3
      motes_per_charge: 1
      each_mote:
        destination:
          visible_to_user: true
          range_ft: 60
        area:
          shape: cube
          size_ft: 15
          origin: destination
        save:
          ability: dexterity
          dc: 15
          failure:
            damage:
              roll:
                expression: 5d4
                scope: use
              type: radiant
          success:
            damage_scale:
              numerator: 1
              denominator: 2
    equipment_state: worn_on_finger_or_similar_digit
    attunement:
      required: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_rings
    - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
splat_contracts:
- id: dnd2024:item_ring_of_shooting_stars
  inputs:
    actor:
      type: participant
      required: true
    source_item:
      type: item_instance
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
    choice:
      required: false
      type: choice
    event:
      required: false
      type: event
    campaign:
      required: false
      type: campaign
  outputs:
  - item_state
  - effects
  - resolutions
  events:
  - spell.cast_pending
  - action.magic_pending
  - action.bonus_pending
  - concentration.ended
  - position.changed
  - time.dawn
  - time.elapsed
  - attunement.changed
  capabilities:
  - inventory
  - effects
  - spellcasting
  - resources_and_time
  - roll_resolution
  - spatial_state
  - participant_state
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 287fc24ebc2bed2186c881bea3b5c3fa2e377a0aab976976623481565346b472
  source_path: builtin/srd-5.2.1/magic_item/ring_of_shooting_stars.md
  pages:
  - 238
  - 239
  engine_gaps: &id001
  - dnd2024:item_ring_of_shooting_stars
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 3416cb0ef305241424c33f90f713eb4d4849555281c254da460abf78895d227e
splat_clauses:
- key: source_rules
  text: '# Ring of Shooting Stars Ring, Very Rare (Requires Attunement) You can cast Dancing Lights or
    Light from the ring. The ring has 6 charges and regains 1d6 expended charges daily at dawn. You can
    expend its charges to use the properties below. Faerie Fire. You can expend 1 charge to cast Faerie
    Fire from the ring. Lightning Spheres. You can expend 2 charges as a Magic action to create up to
    four 3-foot-diameter spheres of lightning. Each sphere appears in an unoccupied space you can see
    within 120 feet of yourself. The spheres last as long as you maintain Concentration, up to 1 minute.
    Each sphere sheds Dim Light in a 30-foot radius. As a Bonus Action, you can move each sphere up to
    30 feet, but no farther than 120 feet away from yourself. The first time the sphere comes within 5
    feet of a creature other than you that isn’t behind Total Cover, the sphere discharges lightning at
    that creature and disappears. That creature makes a DC 15 Dexterity saving throw. On a failed save,
    the creature takes Lightning damage based on the number of spheres you created, as shown in the following
    table. On a successful save, the creature takes half as much damage. Number of Lightning Number of
    Lightning Spheres Damage Spheres Damage 1 4d12 3 2d6 2 5d4 4 2d4 Shooting Stars. You can expend 1
    to 3 charges as a Magic action. For every charge you expend, you launch a glowing mote of light from
    the ring at a point you can see within 60 feet of yourself. Each creature in a 15-foot Cube originating
    from that point is showered in sparks and makes a DC 15 Dexterity saving throw, taking 5d4 Radiant
    damage on a failed save or half as much damage on a successful one.'
  sha256: 4efd2033e42a4521d5ecf6e1ab1cde03114202c5ad0592d900002333c728b6a3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ring of Shooting Stars

<!-- SRD page 238 -->

Ring, Very Rare (Requires Attunement)\
You can cast Dancing Lights or Light from the ring.\
The ring has 6 charges and regains 1d6 expended\
charges daily at dawn. You can expend its charges\
to use the properties below.\
Faerie Fire. You can expend 1 charge to cast Fa-\
erie Fire from the ring.\
Lightning Spheres. You can expend 2 charges as\
a Magic action to create up to four 3-foot-diameter\
spheres of lightning.\
Each sphere appears in an unoccupied space you\
can see within 120 feet of yourself. The spheres\

<!-- SRD page 239 -->

last as long as you maintain Concentration, up to 1\
minute. Each sphere sheds Dim Light in a 30-foot\
radius.\
As a Bonus Action, you can move each sphere up\
to 30 feet, but no farther than 120 feet away from\
yourself. The first time the sphere comes within 5\
feet of a creature other than you that isn’t behind\
Total Cover, the sphere discharges lightning at that\
creature and disappears. That creature makes a\
DC 15 Dexterity saving throw. On a failed save, the\
creature takes Lightning damage based on the num-\
ber of spheres you created, as shown in the follow-\
ing table. On a successful save, the creature takes\
half as much damage.\
Number of  Lightning Number of  Lightning\
Spheres Damage Spheres Damage\
1 4d12 3 2d6\
2 5d4 4 2d4\
Shooting Stars. You can expend 1 to 3 charges as\
a Magic action. For every charge you expend, you\
launch a glowing mote of light from the ring at a\
point you can see within 60 feet of yourself. Each\
creature in a 15-foot Cube originating from that\
point is showered in sparks and makes a DC 15\
Dexterity saving throw, taking 5d4 Radiant damage\
on a failed save or half as much damage on a suc-\
cessful one.\

Original layout: [[../../srd-5.2.1/pages/238|page 238]], [[../../srd-5.2.1/pages/239|page 239]].
