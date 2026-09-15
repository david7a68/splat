---
splat_schema: 1
splat_id: d1142589-53f6-57af-acbc-8959df188421
splat_kind: item
splat_key: dnd2024:items/staff_of_thunder_and_lightning
splat_name: Staff of Thunder and Lightning
splat_origin:
  key: dnd2024:reference/magic_item/staff_of_thunder_and_lightning
splat_sources:
- title: SRD 5.2.1
  locator: pp. 247, 248
  url: https://www.dndbeyond.com/srd
splat_item:
  category: staff
  rarity: very_rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/staff_of_thunder_and_lightning
  implementation: dnd2024:item_staff_of_thunder_and_lightning
  parameters:
    weapon:
      key: dnd2024:equipment/quarterstaff
    attack_roll_bonus: 2
    damage_roll_bonus: 2
    scope: attacks_with_source_item
    properties_each_independent_uses:
      maximum: 1
      initial: 1
      recovery:
        event: dawn
        amount:
          expression: '1'
          scope: use
        cap: maximum
    lightning:
      trigger: melee_hit_with_staff
      action: none
      damage:
        roll:
          expression: 2d6
          scope: attack
        type: lightning
    thunder:
      trigger: melee_hit_with_staff
      action: none
      audible_ft: 300
      save:
        ability: constitution
        dc: 17
        failure:
          condition: stunned
          duration:
            until: end_of_user_next_turn
        success:
          effect: none
    thunder_and_lightning:
      trigger: immediately_after_melee_hit_with_staff
      action: bonus_action
      apply_properties:
      - lightning
      - thunder
      consume_daily_uses:
      - thunder_and_lightning
      consume_other_daily_uses: false
    lightning_strike:
      action: magic
      area:
        shape: line
        width_ft: 5
        length_ft: 120
      save:
        ability: dexterity
        dc: 17
        failure:
          damage:
            roll:
              expression: 9d6
              scope: use
            type: lightning
        success:
          damage_scale:
            numerator: 1
            denominator: 2
    thunderclap:
      action: magic
      audible_ft: 600
      area:
        shape: emanation
        radius_ft: 60
        origin: user
      save:
        ability: constitution
        dc: 17
        failure:
          damage:
            roll:
              expression: 2d6
              scope: use
            type: thunder
          condition: deafened
          duration:
            amount: 1
            unit: minute
        success:
          damage_scale:
            numerator: 1
            denominator: 2
          condition: null
    attunement:
      required: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_staffs
splat_contracts:
- id: dnd2024:item_staff_of_thunder_and_lightning
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
  - attack.hit
  - action.magic_pending
  - action.bonus_pending
  - time.dawn
  - turn.ended
  - time.elapsed
  - attunement.changed
  capabilities:
  - inventory
  - effects
  - resources_and_time
  - roll_resolution
  - spatial_state
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: fe4c3d0b9fcf9e2eb10c832752d73de0ad7778314a29ccc5f25c2eaff48b2737
  source_path: builtin/srd-5.2.1/magic_item/staff_of_thunder_and_lightning.md
  pages:
  - 247
  - 248
  engine_gaps: &id001
  - dnd2024:item_staff_of_thunder_and_lightning
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 31e9398b64c2f862399d3b72515180180128c774af95864ca668c0f68df8b579
splat_clauses:
- key: source_rules
  text: '# Staff of Thunder and Lightning Staff, Very Rare (Requires Attunement) This staff can be wielded
    as a magic Quarterstaff that grants a +2 bonus to attack rolls and damage rolls made with it. It also
    has the following additional properties. Once one of these properties is used, it can’t be used again
    until the next dawn. Lightning. When you hit with a melee attack using the staff, you can cause the
    target to take an extra 2d6 Lightning damage (no action required). Thunder. When you hit with a melee
    attack using the staff, you can cause the staff to emit a crack of thunder audible out to 300 feet
    (no action required). The target you hit must succeed on a DC 17 Constitution saving throw or have
    the Stunned condition until the end of your next turn. Thunder and Lightning. Immediately after you
    hit with a melee attack using the staff, you can take a Bonus Action to use the Lightning and Thunder
    properties (see above) at the same time. Doing so doesn’t expend the daily use of those properties,
    only the use of this one. Lightning Strike. You can take a Magic action to cause a bolt of lightning
    to leap from the staff’s tip in a Line that is 5 feet wide and 120 feet long. Each creature in that
    Line makes a DC 17 Dexterity saving throw, taking 9d6 Lightning damage on a failed save or half as
    much damage on a successful one. Thunderclap. You can take a Magic action to cause the staff to produce
    a thunderclap audible out to 600 feet. Every creature within a 60-foot Emanation originating from
    you makes a DC 17 Constitution saving throw. On a failed save, a creature takes 2d6 Thunder damage
    and has the Deafened condition for 1 minute. On a successful save, a creature takes half as much damage
    only.'
  sha256: 350cf6d9c5d09c8a8f7cf7aa2dc59681d6f6bf21772519c0e8ed3ed06ae99080
  status: contract_specified
splat_requires_mechanics: *id001
---

# Staff of Thunder and Lightning

<!-- SRD page 247 -->

Staff, Very Rare (Requires Attunement)\
This staff can be wielded as a magic Quarterstaff\
that grants a +2 bonus to attack rolls and damage\
rolls made with it. It also has the following addi-\
tional properties. Once one of these properties is\
used, it can’t be used again until the next dawn.\
Lightning. When you hit with a melee attack\
using the staff, you can cause the target to take an\
extra 2d6 Lightning damage (no action required).\
Thunder. When you hit with a melee attack using\
the staff, you can cause the staff to emit a crack of\
thunder audible out to 300 feet (no action required).\
The target you hit must succeed on a DC 17 Consti-\
tution saving throw or have the Stunned condition\
until the end of your next turn.\

<!-- SRD page 248 -->

Thunder and Lightning. Immediately after you\
hit with a melee attack using the staff, you can take\
a Bonus Action to use the Lightning and Thunder\
properties (see above) at the same time. Doing so\
doesn’t expend the daily use of those properties,\
only the use of this one.\
Lightning Strike. You can take a Magic action to\
cause a bolt of lightning to leap from the staff’s tip\
in a Line that is 5 feet wide and 120 feet long. Each\
creature in that Line makes a DC 17 Dexterity sav-\
ing throw, taking 9d6 Lightning damage on a failed\
save or half as much damage on a successful one.\
Thunderclap. You can take a Magic action to\
cause the staff to produce a thunderclap audible out\
to 600 feet. Every creature within a 60-foot Emana-\
tion originating from you makes a DC 17 Constitu-\
tion saving throw. On a failed save, a creature takes\
2d6 Thunder damage and has the Deafened condi-\
tion for 1 minute. On a successful save, a creature\
takes half as much damage only.\

Original layout: [[../../srd-5.2.1/pages/247|page 247]], [[../../srd-5.2.1/pages/248|page 248]].
