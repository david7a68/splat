---
splat_schema: 1
splat_id: 40c57379-6f88-5f05-9479-5208a5cd0e9b
splat_kind: item
splat_key: dnd2024:items/dancing_sword
splat_name: Dancing Sword
splat_origin:
  key: dnd2024:reference/magic_item/dancing_sword
splat_sources:
- title: SRD 5.2.1
  locator: pp. 217
  url: https://www.dndbeyond.com/srd
splat_item:
  category: weapon
  rarity: very_rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/dancing_sword
  implementation: dnd2024:item_dancing_sword
  parameters:
    activate:
      action: bonus_action
      toss: true
      fly_ft: 30
      attack:
        target_count: 1
        target_range_from_weapon_ft: 5
        attack_roll: actor.weapon_attack
        damage_modifier: actor.weapon_ability_modifier
    hover_command:
      action: bonus_action
      move_up_to_ft: 30
      destination_range_from_actor_ft: 30
      optional_attack:
        target_count: 1
        target_range_from_weapon_ft: 5
        attack_roll: actor.weapon_attack
        damage_modifier: actor.weapon_ability_modifier
    after_attack_count: 4
    return_behavior:
      to: actor_hand
      no_free_hand: drop_in_actor_space
      obstructed_path: move_closest_then_drop
    hover_ends_on:
    - grasped
    - distance_from_actor_over_30_ft
    attunement_required: true
splat_contracts:
- id: dnd2024:item_dancing_sword
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
  - attack.resolved
  - item.grasped
  - movement.completed
  - item.attunement_completed
  - item.attunement_ended
  capabilities:
  - item_effects
  - dice
  - damage
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 398807733f38f27b008ede8cd0712b3e0865591e2f13325fc06bece8ee04cf87
  source_path: builtin/srd-5.2.1/magic_item/dancing_sword.md
  pages:
  - 217
  engine_gaps: &id001
  - dnd2024:item_dancing_sword
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 64826928b273c7b32b68fbf09965411e2bddccdc5be4e8b24f2057f634aa5685
splat_clauses:
- key: source_rules
  text: '# Dancing Sword Weapon (Greatsword, Longsword, Rapier, Scimitar, or Shortsword), Very Rare (Requires
    Attunement) You can take a Bonus Action to toss this magic weapon into the air. When you do so, the
    weapon begins to hover, flies up to 30 feet, and attacks one creature of your choice within 5 feet
    of itself. The weapon uses your attack roll and adds your ability modifier to damage rolls. While
    the weapon hovers, you can take a Bonus Action to cause it to fly up to 30 feet to another spot within
    30 feet of you. As part of the same Bonus Action, you can cause the weapon to attack one creature
    within 5 feet of the weapon. After the hovering weapon attacks for the fourth time, it flies back
    to you and tries to return to your hand. If you have no hand free, the weapon falls to the ground
    in your space. If the weapon has no unobstructed path to you, it moves as close to you as it can and
    then falls to the ground. It also ceases to hover if you grasp it or are more than 30 feet away from
    it.'
  sha256: 0e45e88dc9bbc2d79eef0695dca73675eeee97df1585c3c0354c473bcdd77f81
  status: contract_specified
splat_requires_mechanics: *id001
---

# Dancing Sword

<!-- SRD page 217 -->

Weapon (Greatsword, Longsword, Rapier, Scimitar, or\
Shortsword), Very Rare (Requires Attunement)\
You can take a Bonus Action to toss this magic\
weapon into the air. When you do so, the weapon\
begins to hover, flies up to 30 feet, and attacks one\
creature of your choice within 5 feet of itself. The\
weapon uses your attack roll and adds your ability\
modifier to damage rolls.\
While the weapon hovers, you can take a Bonus\
Action to cause it to fly up to 30 feet to another spot\
within 30 feet of you. As part of the same Bonus Ac-\
tion, you can cause the weapon to attack one crea-\
ture within 5 feet of the weapon.\
After the hovering weapon attacks for the fourth\
time, it flies back to you and tries to return to your\
hand. If you have no hand free, the weapon falls to\
the ground in your space. If the weapon has no un-\
obstructed path to you, it moves as close to you as\
it can and then falls to the ground. It also ceases to\
hover if you grasp it or are more than 30 feet away\
from it.\

Original layout: [[../../srd-5.2.1/pages/217|page 217]].
