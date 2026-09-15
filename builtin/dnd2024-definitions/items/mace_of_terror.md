---
splat_schema: 1
splat_id: eca6616c-5dd6-5a04-b836-c3e227c158ae
splat_kind: item
splat_key: dnd2024:items/mace_of_terror
splat_name: Mace of Terror
splat_origin:
  key: dnd2024:reference/magic_item/mace_of_terror
splat_sources:
- title: SRD 5.2.1
  locator: pp. 229
  url: https://www.dndbeyond.com/srd
splat_item:
  category: weapon
  rarity: rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/mace_of_terror
  implementation: dnd2024:item_mace_of_terror
  parameters:
    equipment_state: held
    charges:
      maximum: 3
      initial: 3
      recovery:
        event: dawn
        amount:
          expression: 1d3
          scope: use
        cap: maximum
    terror:
      action: magic
      charge_cost: 1
      targets:
        chooser: user
        range_ft: 30
      save:
        ability: wisdom
        dc: 15
        failure:
          condition: frightened
          duration:
            amount: 1
            unit: minute
        success:
          effect: none
      while_frightened:
        movement: as_far_away_from_user_as_possible
        opportunity_attacks: false
        actions_allowed:
        - dash
        - escape_movement_preventing_effect
        if_no_movement_possible: dodge_allowed
      repeat_save:
        event: target_turn_end
        ability: wisdom
        dc: 15
        success: end_effect
    attunement:
      required: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_weapons
splat_contracts:
- id: dnd2024:item_mace_of_terror
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
  - action.magic_pending
  - turn.started
  - turn.ended
  - time.dawn
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
  source_sha256: d6910163abdf07ee4887be45efcf35971755056ac0c99f4f9e52f051270ff1b0
  source_path: builtin/srd-5.2.1/magic_item/mace_of_terror.md
  pages:
  - 229
  engine_gaps: &id001
  - dnd2024:item_mace_of_terror
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: ed5d9346da03aad5d902ce84421321bec3fe32741d3ab16867d8af6c495a866b
splat_clauses:
- key: source_rules
  text: '# Mace of Terror Weapon (Mace), Rare (Requires Attunement) This magic weapon has 3 charges and
    regains 1d3 expended charges daily at dawn. While holding the weapon, you can take a Magic action
    and expend 1 charge to release a wave of terror from it. Each creature of your choice within 30 feet
    of you must succeed on a DC 15 Wisdom saving throw or have the Frightened condition for 1 minute.
    While Frightened in this way, a creature must spend its turns trying to move as far away from you
    as it can, and it can’t make Opportunity Attacks. For its action, it can use only the Dash action
    or try to escape from an effect that prevents it from moving. If it has nowhere it can move, the creature
    can take the Dodge action. At the end of each of its turns, a creature repeats the save, ending the
    effect on itself on a success.'
  sha256: cc1c4773a38124859ce1304fca91fee48e6c33a4d0e3ebee7bdc13ff669837d5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Mace of Terror

<!-- SRD page 229 -->

Weapon (Mace), Rare (Requires Attunement)\
This magic weapon has 3 charges and regains 1d3\
expended charges daily at dawn. While holding\
the weapon, you can take a Magic action and ex-\
pend 1 charge to release a wave of terror from it.\
Each creature of your choice within 30 feet of you\
must succeed on a DC 15 Wisdom saving throw or\
have the Frightened condition for 1 minute. While\
Frightened in this way, a creature must spend its\
turns trying to move as far away from you as it can,\
and it can’t make Opportunity Attacks. For its ac-\
tion, it can use only the Dash action or try to escape\
from an effect that prevents it from moving. If it\
has nowhere it can move, the creature can take the\
Dodge action. At the end of each of its turns, a crea-\
ture repeats the save, ending the effect on itself on\
a success.\

Original layout: [[../../srd-5.2.1/pages/229|page 229]].
