---
splat_schema: 1
splat_id: 01d0960f-fe28-5f4b-9d35-5797d59106a3
splat_kind: item
splat_key: dnd2024:items/rod_of_lordly_might
splat_name: Rod of Lordly Might
splat_origin:
  key: dnd2024:reference/magic_item/rod_of_lordly_might
splat_sources:
- title: SRD 5.2.1
  locator: pp. 241, 242
  url: https://www.dndbeyond.com/srd
splat_item:
  category: rod
  rarity: legendary
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/rod_of_lordly_might
  implementation: dnd2024:item_rod_of_lordly_might
  parameters:
    normal_form:
      weapon:
        key: dnd2024:equipment/mace
      attack_bonus: 3
      damage_bonus: 3
      magical: true
    buttons:
      action: bonus_action
      duration: until_same_or_different_button_pressed
      repeat_same_button: normal_form
      options:
        '1':
          weapon_choice:
          - key: dnd2024:equipment/longsword
          - key: dnd2024:equipment/shortsword
          magical: true
          extra_hit_damage:
            roll:
              dice: 2
              sides: 6
              bonus: 0
            type: fire
          light:
            bright_radius_ft: 40
            additional_dim_ft: 40
          attack_bonus: 0
          damage_bonus: 0
        '2':
          weapon:
            key: dnd2024:equipment/battleaxe
          attack_bonus: 3
          damage_bonus: 3
          magical: true
        '3':
          weapon:
            key: dnd2024:equipment/spear
          attack_bonus: 3
          damage_bonus: 3
          magical: true
          length_ft: 6
        '4':
          kind: climbing_pole
          length:
            maximum_ft: 50
            chooser: user
          buttons_remain_reachable: true
          anchors:
            bottom_spikes: 1
            top_hooks: 3
            maximum_surface_hardness: granite
          rungs:
            length_inches: 3
            spacing_ft: 1
          maximum_load_lb: 4000
          revert_if:
          - overloaded
          - not_solidly_anchored
        '5':
          kind: handheld_battering_ram
          checks:
            ability: strength
            skill: athletics
            bonus: 10
            purpose: break_doors_barricades_or_barriers
        '6':
          form: normal
          information:
          - magnetic_north
          - approximate_depth_or_height_relative_to_ground
          north_requires: magnetic_north_exists
    drain_life:
      trigger: melee_hit_with_rod_against_creature
      optional: true
      resource:
        uses: 1
        recovery: dawn
      save:
        ability: constitution
        dc: 17
      failure:
        damage:
          roll:
            dice: 4
            sides: 6
            bonus: 0
          type: necrotic
        user_healing:
          fraction_of_necrotic_damage: 0.5
    paralyze:
      trigger: melee_hit_with_rod_against_creature
      optional: true
      resource:
        uses: 1
        recovery: dawn
      save:
        ability: constitution
        dc: 17
      failure:
        condition: paralyzed
        duration:
          value: 1
          unit: minute
      repeat_save: target_turn_end
      success: end_condition
    terrify:
      requires: held
      action: magic
      resource:
        uses: 1
        recovery: dawn
      targets:
        kind: creatures
        seen: true
        range_ft: 30
        count: all
      save:
        ability: wisdom
        dc: 17
      failure:
        condition: frightened
        duration:
          value: 1
          unit: minute
      repeat_save: target_turn_end
      success: end_condition
    attunement:
      required: true
splat_contracts:
- id: dnd2024:item_rod_of_lordly_might
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
    locations:
      required: false
      type: location_list
    event:
      required: false
      type: event
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
  source_sha256: 06fb26beebb65bb04bff915674f97931252833b1656648f14b74905f06aa2657
  source_path: builtin/srd-5.2.1/magic_item/rod_of_lordly_might.md
  pages:
  - 241
  - 242
  engine_gaps: &id001
  - dnd2024:item_rod_of_lordly_might
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: 923dc553e25c1fb7b982f48681cb7e0e309afed75186315e570d2c944ae8b80b
splat_clauses:
- key: source_rules
  text: '# Rod of Lordly Might Rod, Legendary (Requires Attunement) This rod has a flanged head, and it
    functions as a magic Mace that grants a +3 bonus to attack rolls and damage rolls made with it. The
    rod has properties associated with six different buttons that are set in a row along the haft. It
    has three other properties as well, detailed below. Buttons. You can press one of the following buttons
    as a Bonus Action; a button’s effect lasts until you push a different button or until you push the
    same button again, which causes the rod to revert to its normal form: Button 1. A fiery blade sprouts
    from the end opposite the rod’s flanged head. The flames shed Bright Light in a 40-foot radius and
    Dim Light for an additional 40 feet, and the blade functions as a magic Longsword or Shortsword (your
    choice) that deals an extra 2d6 Fire damage on a hit. Button 2. The rod’s flanged head folds down
    and two crescent-shaped blades spring out, transforming the rod into a magic Battleaxe that grants
    a +3 bonus to attack rolls and damage rolls made with it. Button 3. The rod’s flanged head folds down,
    a spear point springs from the rod’s tip, and the rod’s handle lengthens into a 6-foot haft, transforming
    the rod into a magic Spear that grants a +3 bonus to attack rolls and damage rolls made with it. Button
    4. The rod transforms into a climbing pole up to 50 feet long (you specify the length), though the
    rod’s buttons remain within your reach. In surfaces as hard as granite, a spike at the bottom and
    three hooks at the top anchor the pole. Horizontal bars 3 inches long fold out from the sides, 1 foot
    apart, forming a ladder. The pole can bear up to 4,000 pounds. More weight or lack of solid anchoring
    causes the rod to revert to its normal form. Button 5. The rod transforms into a handheld battering
    ram and grants its user a +10 bonus to Strength (Athletics) checks made to break through doors, barricades,
    and other barriers. Button 6. The rod assumes or remains in its normal form and indicates magnetic
    north. (Nothing happens if this function of the rod is used in a location that has no magnetic north.)
    The rod also gives you knowledge of your approximate depth beneath the ground or your height above
    it. Drain Life. When you hit a creature with a melee attack using the rod, you can force the target
    to make a DC 17 Constitution saving throw. On a failed save, the target takes an extra 4d6 Necrotic
    damage, and you regain a number of Hit Points equal to half that Necrotic damage. Once used, this
    property can’t be used again until the next dawn. Paralyze. When you hit a creature with a melee attack
    using the rod, you can force the target to make a DC 17 Constitution saving throw. On a failed save,
    the target has the Paralyzed condition for 1 minute. The target repeats the save at the end of each
    of its turns, ending the effect on a success. Once used, this property can’t be used again until the
    next dawn. Terrify. While holding the rod, you can take a Magic action to force each creature you
    can see within 30 feet of yourself to make a DC 17 Wisdom saving throw. On a failed save, a target
    has the Frightened condition for 1 minute. A Frightened target repeats the save at the end of each
    of its turns, ending the effect on itself on a success. Once used, this property can’t be used again
    until the next dawn.'
  sha256: 85d57a040490b71686dccf7d350d28a513e2eaaa678c61e9cad5c051dec50dac
  status: contract_specified
splat_requires_mechanics: *id001
---

# Rod of Lordly Might

<!-- SRD page 241 -->

Rod, Legendary (Requires Attunement)\
This rod has a flanged head, and it functions as a\
magic Mace that grants a +3 bonus to attack rolls\
and damage rolls made with it. The rod has proper-\
ties associated with six different buttons that are\
set in a row along the haft. It has three other prop-\
erties as well, detailed below.\
Buttons. You can press one of the following but-\
tons as a Bonus Action; a button’s effect lasts until\
you push a different button or until you push the\
same button again, which causes the rod to revert\
to its normal form:\
Button 1. A fiery blade sprouts from the end op-\
posite the rod’s flanged head. The flames shed\
Bright Light in a 40-foot radius and Dim Light for\
an additional 40 feet, and the blade functions as\
a magic Longsword or Shortsword (your choice)\
that deals an extra 2d6 Fire damage on a hit.\
Button 2. The rod’s flanged head folds down and\
two crescent-shaped blades spring out, trans-\
forming the rod into a magic Battleaxe that grants\
a +3 bonus to attack rolls and damage rolls made\
with it.\
Button 3. The rod’s flanged head folds down, a\
spear point springs from the rod’s tip, and the\
rod’s handle lengthens into a 6-foot haft, trans-\
forming the rod into a magic Spear that grants a\
+3 bonus to attack rolls and damage rolls made\
with it.\
Button 4. The rod transforms into a climbing pole\
up to 50 feet long (you specify the length), though\
the rod’s buttons remain within your reach. In\
surfaces as hard as granite, a spike at the bot-\
tom and three hooks at the top anchor the pole.\
Horizontal bars 3 inches long fold out from the\
sides, 1 foot apart, forming a ladder. The pole can\
bear up to 4,000 pounds. More weight or lack of\
solid anchoring causes the rod to revert to its\
normal form.\
Button 5. The rod transforms into a handheld\
battering ram and grants its user a +10 bonus\
to Strength (Athletics) checks made to break\
through doors, barricades, and other barriers.\
Button 6. The rod assumes or remains in its nor-\
mal form and indicates magnetic north. (Nothing\
happens if this function of the rod is used in a\

<!-- SRD page 242 -->

location that has no magnetic north.) The rod also\
gives you knowledge of your approximate depth\
beneath the ground or your height above it.\
Drain Life. When you hit a creature with a melee\
attack using the rod, you can force the target to\
make a DC 17 Constitution saving throw. On a failed\
save, the target takes an extra 4d6 Necrotic dam-\
age, and you regain a number of Hit Points equal to\
half that Necrotic damage. Once used, this property\
can’t be used again until the next dawn.\
Paralyze. When you hit a creature with a melee\
attack using the rod, you can force the target to\
make a DC 17 Constitution saving throw. On a failed\
save, the target has the Paralyzed condition for 1\
minute. The target repeats the save at the end of\
each of its turns, ending the effect on a success.\
Once used, this property can’t be used again until\
the next dawn.\
Terrify. While holding the rod, you can take a\
Magic action to force each creature you can see\
within 30 feet of yourself to make a DC 17 Wisdom\
saving throw. On a failed save, a target has the\
Frightened condition for 1 minute. A Frightened\
target repeats the save at the end of each of its\
turns, ending the effect on itself on a success. Once\
used, this property can’t be used again until the\
next dawn.\

Original layout: [[../../srd-5.2.1/pages/241|page 241]], [[../../srd-5.2.1/pages/242|page 242]].
