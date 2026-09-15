---
splat_schema: 1
splat_id: cecad3c6-628e-59ac-8143-0156183f5cb2
splat_kind: item
splat_key: dnd2024:items/rope_of_entanglement
splat_name: Rope of Entanglement
splat_origin:
  key: dnd2024:reference/magic_item/rope_of_entanglement
splat_sources:
- title: SRD 5.2.1
  locator: pp. 242, 243
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/rope_of_entanglement
  implementation: dnd2024:item_rope_of_entanglement
  parameters:
    length_ft: 30
    equipped: holding_one_end
    activate:
      action: magic
      command: true
      target:
        visible: true
        range_ft: 20
        creature_count: 1
      save:
        ability: dexterity
        dc: 15
        failure:
          condition: restrained
    release_options:
    - trigger: user_lets_go
      rope_location: target_space
      rope_state: coiled
    - action: bonus_action
      repeat_command: true
      rope_location: user_hand
      rope_state: coiled
    escape:
      action: action
      check_choice:
      - ability: strength
        skill: athletics
        dc: 15
      - ability: dexterity
        skill: acrobatics
        dc: 15
      success: end_restrained
      user_reaction:
        requires: holding_rope
        command: coil_into_user_hand
      otherwise_rope_location: target_space
    object_statistics:
      ac: 20
      hp: 20
      damage_immunities:
      - poison
      - psychic
      healing:
        hp: 1
        interval:
          amount: 5
          unit: minutes
        requires_hp_minimum: 1
      zero_hp: destroyed
    restrained_ends_on:
    - rope_destroyed
    - release
    - successful_escape_check
    attunement_required: false
splat_contracts:
- id: dnd2024:item_rope_of_entanglement
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
  - item.grip_changed
  - item.hp_changed
  - time.elapsed
  capabilities:
  - item_effects
  - spatial_targeting
  - damage
  - healing
  - conditions
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 9f28cfdf8e8a3d0cc8c23d7cfffbae2d1ed00e137632316f89304e6354be000b
  source_path: builtin/srd-5.2.1/magic_item/rope_of_entanglement.md
  pages:
  - 242
  - 243
  engine_gaps: &id001
  - dnd2024:item_rope_of_entanglement
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 2e9acd13801caea25255ed6dbdb086d3102c381501cb9f86ddac837f1998bdfc
splat_clauses:
- key: source_rules
  text: '# Rope of Entanglement Wondrous Item, Rare This rope is 30 feet long. While holding one end of
    the rope, you can take a Magic action to command the other end to dart forward and entangle one creature
    you can see within 20 feet of yourself. The target must succeed on a DC 15 Dexterity saving throw
    or have the Restrained condition. You can release the target by letting go of your end of the rope
    (causing the rope to coil up in the target’s space) or by using a Bonus Action to repeat the command
    (causing the rope to coil up in your hand). A target Restrained by the rope can take an action to
    make its choice of a DC 15 Strength (Athletics) or Dexterity (Acrobatics) check. On a successful check,
    the target is no longer Restrained by the rope. If you’re still holding onto the rope when a target
    escapes from it, you can take a Reaction to command the rope to coil up in your hand; otherwise, the
    rope coils up in the target’s space. The rope has AC 20, HP 20, and Immunity to Poison and Psychic
    damage. It regains 1 Hit Point every 5 minutes as long as it has at least 1 Hit Point. If the rope
    drops to 0 Hit Points, it is destroyed.'
  sha256: 37482f8f1c13affc497c01677886ca41bd3e4f5df5f77bbdef5a6b439f28f718
  status: contract_specified
splat_requires_mechanics: *id001
---

# Rope of Entanglement

<!-- SRD page 242 -->

Wondrous Item, Rare\
This rope is 30 feet long. While holding one end of\
the rope, you can take a Magic action to command\
the other end to dart forward and entangle one\
creature you can see within 20 feet of yourself. The\
target must succeed on a DC 15 Dexterity saving\
throw or have the Restrained condition. You can re-\
lease the target by letting go of your end of the rope\
(causing the rope to coil up in the target’s space)\
or by using a Bonus Action to repeat the command\
(causing the rope to coil up in your hand).\

<!-- SRD page 243 -->

A target Restrained by the rope can take an action\
to make its choice of a DC 15 Strength (Athletics) or\
Dexterity (Acrobatics) check. On a successful check,\
the target is no longer Restrained by the rope. If\
you’re still holding onto the rope when a target es-\
capes from it, you can take a Reaction to command\
the rope to coil up in your hand; otherwise, the rope\
coils up in the target’s space.\
The rope has AC 20, HP 20, and Immunity to Poi-\
son and Psychic damage. It regains 1 Hit Point every\
5 minutes as long as it has at least 1 Hit Point. If the\
rope drops to 0 Hit Points, it is destroyed.\

Original layout: [[../../srd-5.2.1/pages/242|page 242]], [[../../srd-5.2.1/pages/243|page 243]].
