---
splat_schema: 1
splat_id: fe0188fc-b4cc-57ee-aab2-bbcd2f38a29d
splat_kind: item
splat_key: dnd2024:items/hammer_of_thunderbolts
splat_name: Hammer of Thunderbolts
splat_origin:
  key: dnd2024:reference/magic_item/hammer_of_thunderbolts
splat_sources:
- title: SRD 5.2.1
  locator: pp. 224
  url: https://www.dndbeyond.com/srd
splat_item:
  category: weapon
  rarity: legendary
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/hammer_of_thunderbolts
  implementation: dnd2024:item_hammer_of_thunderbolts
  parameters:
    attack_roll_bonus: 1
    damage_roll_bonus: 1
    scope: attacks_with_source_item
    charges:
      maximum: 5
      initial: 5
      recovery:
        event: dawn
        amount:
          expression: 1d4+1
          scope: use
        cap: maximum
    throw:
      charge_cost: 1
      economy: ranged_weapon_attack
      property: thrown
      normal_range_ft: 20
      long_range_ft: 60
      on_hit:
        audible_ft: 300
        targets:
          origin: hit_target
          radius_ft: 30
          include_hit_target: true
          exclude_user: true
        save:
          ability: constitution
          dc: 17
          failure:
            condition: stunned
            duration:
              until: end_of_user_next_turn
          success:
            effect: none
      after_hit_or_miss: returns_to_hand
    giants_bane:
      requires:
        weapon_attuned: true
        worn_and_attuned_any:
        - key: dnd2024:items/belt_of_giant_strength
        - key: dnd2024:items/gauntlets_of_ogre_power
      on_natural_20:
        target_type: giant
        save:
          ability: constitution
          dc: 17
          failure:
            dead: true
          success:
            effect: none
      other_item_strength_grant:
        increase: 4
        maximum: 30
    attunement:
      required: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_weapons
splat_contracts:
- id: dnd2024:item_hammer_of_thunderbolts
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
  - attack.roll_pending
  - attack.hit
  - attack.miss
  - time.dawn
  - equipment.changed
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
  source_sha256: 976ef6d639983de249d9e6a4fca081f385374066ed717c3a5dc3bc15685a4d12
  source_path: builtin/srd-5.2.1/magic_item/hammer_of_thunderbolts.md
  pages:
  - 224
  engine_gaps: &id001
  - dnd2024:item_hammer_of_thunderbolts
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 517e87869ac7a8709dcb427a236712849d27195808c2897b803ae4aa2331439c
splat_clauses:
- key: source_rules
  text: '# Hammer of Thunderbolts Weapon (Maul or Warhammer), Legendary (Requires Attunement) You gain
    a +1 bonus to attack rolls and damage rolls made with this magic weapon. The weapon has 5 charges.
    You can expend 1 charge and make a ranged attack with the weapon, hurling it as if it had the Thrown
    property with a normal range of 20 feet and a long range of 60 feet. If the attack hits, the weapon
    unleashes a thunderclap audible out to 300 feet. The target and every creature within 30 feet of it
    other than you must succeed on a DC 17 Constitution saving throw or have the Stunned condition until
    the end of your next turn. Immediately after hitting or missing, the weapon flies back to your hand.
    The weapon regains 1d4 + 1 expended charges daily at dawn. Giant’s Bane. While you are attuned to
    the weapon and wearing either a Belt of Giant Strength or Gauntlets of Ogre Power to which you are
    also attuned, you gain the following benefits: Giants’ Bane. When you roll a 20 on the d20 for an
    attack roll made with this weapon against a Giant, the creature must succeed on a DC 17 Constitution
    saving throw or die. Might of Giants. The Strength score bestowed by your Belt of Giant Strength or
    Gauntlets of Ogre Power increases by 4, to a maximum of 30.'
  sha256: 27f4fdc42f3b785e7c1b65b23e75cf0cbff1e271ab8d06dd8e37034c74080d20
  status: contract_specified
splat_requires_mechanics: *id001
---

# Hammer of Thunderbolts

<!-- SRD page 224 -->

Weapon (Maul or Warhammer), Legendary (Requires\
Attunement)\
You gain a +1 bonus to attack rolls and damage rolls\
made with this magic weapon.\
The weapon has 5 charges. You can expend 1\
charge and make a ranged attack with the weapon,\
hurling it as if it had the Thrown property with a\
normal range of 20 feet and a long range of 60 feet.\
If the attack hits, the weapon unleashes a thunder-\
clap audible out to 300 feet. The target and every\
creature within 30 feet of it other than you must\
succeed on a DC 17 Constitution saving throw or\
have the Stunned condition until the end of your\
next turn. Immediately after hitting or missing, the\
weapon flies back to your hand. The weapon re-\
gains 1d4 + 1 expended charges daily at dawn.\
Giant’s Bane. While you are attuned to the\
weapon and wearing either a Belt of Giant Strength\
or Gauntlets of Ogre Power to which you are also at-\
tuned, you gain the following benefits:\
Giants’ Bane. When you roll a 20 on the d20 for an\
attack roll made with this weapon against a Giant,\
the creature must succeed on a DC 17 Constitu-\
tion saving throw or die.\
Might of Giants. The Strength score bestowed by\
your Belt of Giant Strength or Gauntlets of Ogre\
Power increases by 4, to a maximum of 30.\

Original layout: [[../../srd-5.2.1/pages/224|page 224]].
