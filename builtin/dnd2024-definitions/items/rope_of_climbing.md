---
splat_schema: 1
splat_id: bd3f526f-3cbb-5518-9533-1c3215a5fe64
splat_kind: item
splat_key: dnd2024:items/rope_of_climbing
splat_name: Rope of Climbing
splat_origin:
  key: dnd2024:reference/magic_item/rope_of_climbing
splat_sources:
- title: SRD 5.2.1
  locator: pp. 242
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/rope_of_climbing
  implementation: dnd2024:item_rope_of_climbing
  parameters:
    length_ft: 60
    weight_capacity_lb: 3000
    ac: 20
    hp: 20
    damage_immunities:
    - poison
    - psychic
    regeneration:
      amount: 1
      interval:
        amount: 5
        unit: minute
      minimum_current_hp: 1
    zero_hp: destroyed
    animate:
      equipment_state: one_end_held
      action: magic
      destination_maximum_distance: rope_length
      movement_ft_on_activation: 10
      movement_ft_at_user_turn_start: 10
      ends_on:
      - destination_reached
      - stop_command
    commands:
    - fasten_to_object
    - unfasten
    - knot
    - unknot
    - coil_for_carrying
    knotted:
      knot_interval_ft: 1
      length_ft: 50
      ability_check_advantage: climb_using_rope
    attunement:
      required: false
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wondrous_items
splat_contracts:
- id: dnd2024:item_rope_of_climbing
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
  - command.issued
  - turn.started
  - time.elapsed
  - participant.hp_changed
  - check.pending
  capabilities:
  - inventory
  - effects
  - resources_and_time
  - roll_resolution
  - spatial_state
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: f39411943f8630885f34a905c001993eb610d6717cc51edc9a686bd9583b80f3
  source_path: builtin/srd-5.2.1/magic_item/rope_of_climbing.md
  pages:
  - 242
  engine_gaps: &id001
  - dnd2024:item_rope_of_climbing
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: f11a218d6df16453a3b695bb98a3b4605da80170470003028ceb79f6d56b734b
splat_clauses:
- key: source_rules
  text: '# Rope of Climbing Wondrous Item, Uncommon This 60-foot length of rope can hold up to 3,000 pounds.
    While holding one end of the rope, you can take a Magic action to command the other end of the rope
    to animate and move toward a destination you choose, up to the rope’s length away from you. That end
    moves 10 feet on your turn when you first command it and 10 feet at the start of each of your subsequent
    turns until reaching its destination or until you tell it to stop. You can also tell the rope to fasten
    itself securely to an object or to unfasten itself, to knot or unknot itself, or to coil itself for
    carrying. If you tell the rope to knot, large knots appear at 1-foot intervals along the rope. While
    knotted, the rope shortens to a 50-foot length and grants Advantage on ability checks made to climb
    using the rope. The rope has AC 20, HP 20, and Immunity to Poison and Psychic damage. It regains 1
    Hit Point every 5 minutes as long as it has at least 1 Hit Point. If the rope drops to 0 Hit Points,
    it is destroyed.'
  sha256: ad004b999c7f4b5c58e796e0b4a45ff63cc7008a8eba5e518679a5d8a6548843
  status: contract_specified
splat_requires_mechanics: *id001
---

# Rope of Climbing

<!-- SRD page 242 -->

Wondrous Item, Uncommon\
This 60-foot length of rope can hold up to 3,000\
pounds. While holding one end of the rope, you can\
take a Magic action to command the other end of the\
rope to animate and move toward a destination you\
choose, up to the rope’s length away from you. That\
end moves 10 feet on your turn when you first com-\
mand it and 10 feet at the start of each of your sub-\
sequent turns until reaching its destination or until\
you tell it to stop. You can also tell the rope to fasten\
itself securely to an object or to unfasten itself, to\
knot or unknot itself, or to coil itself for carrying.\
If you tell the rope to knot, large knots appear at\
1-foot intervals along the rope. While knotted, the\
rope shortens to a 50-foot length and grants Advan-\
tage on ability checks made to climb using the rope.\
The rope has AC 20, HP 20, and Immunity to Poi-\
son and Psychic damage. It regains 1 Hit Point every\
5 minutes as long as it has at least 1 Hit Point. If the\
rope drops to 0 Hit Points, it is destroyed.\

Original layout: [[../../srd-5.2.1/pages/242|page 242]].
