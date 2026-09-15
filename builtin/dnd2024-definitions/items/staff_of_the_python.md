---
splat_schema: 1
splat_id: f4ab86fd-f6e9-5557-9649-abe86bb353b1
splat_kind: item
splat_key: dnd2024:items/staff_of_the_python
splat_name: Staff of the Python
splat_origin:
  key: dnd2024:reference/magic_item/staff_of_the_python
splat_sources:
- title: SRD 5.2.1
  locator: pp. 247
  url: https://www.dndbeyond.com/srd
splat_item:
  category: staff
  rarity: uncommon
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/staff_of_the_python
  implementation: dnd2024:item_staff_of_the_python
  parameters:
    activate:
      action: magic
      throw_range_ft: 10
      unoccupied_space: true
      transform:
        key: dnd2024:creatures/giant_constrictor_snake
    snake:
      controller: user
      initiative: user_initiative
      turn: immediately_after_user
      command:
        action: none
        'on': user_turn
        mental: true
        range_ft: 60
        requires_user_not_incapacitated: true
        choices:
        - action_and_movement
        - general_command
      no_commands: defend_self
    revert:
      action: bonus_action
      command: true
      location: snake_current_space
      heal_snake_to_full: true
      cooldown:
        amount: 1
        unit: hours
    snake_zero_hp:
    - snake_dies
    - revert_to_staff
    - staff_shatters_and_destroyed
    attunement_required: true
splat_contracts:
- id: dnd2024:item_staff_of_the_python
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
  - turn.started
  - hp.changed
  - time.elapsed
  - item.attunement_completed
  - item.attunement_ended
  capabilities:
  - item_effects
  - dice
  - item_recharge
  - spatial_targeting
  - healing
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: d625fd37e9fe46dea336f76db2e6b6fee179ab64a91c541d80fe55fb3812533c
  source_path: builtin/srd-5.2.1/magic_item/staff_of_the_python.md
  pages:
  - 247
  engine_gaps: &id001
  - dnd2024:item_staff_of_the_python
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 51e119701dfd8055c486773e38efe0864438a2c0a6c90526e6b6502a9e60a5af
splat_clauses:
- key: source_rules
  text: '# Staff of the Python Staff, Uncommon (Requires Attunement) As a Magic action, you can throw
    this staff so that it lands in an unoccupied space within 10 feet of you, causing the staff to become
    a Giant Constrictor Snake in that space. The snake is under your control and shares your Initiative
    count, taking its turn immediately after yours. On your turn, you can mentally command the snake (no
    action required) if it is within 60 feet of you and you don’t have the Incapacitated condition. You
    decide what action the snake takes and where it moves during its turn, or you can issue it a general
    command, such as to attack your enemies or guard a location. Absent commands from you, the snake defends
    itself. As a Bonus Action, you can command the snake to revert to staff form in its current space,
    and you can’t use the staff’s property again for 1 hour. If the snake is reduced to 0 Hit Points,
    it dies and reverts to its staff form; the staff then shatters and is destroyed. If the snake reverts
    to staff form before losing all its Hit Points, it regains all of them.'
  sha256: 2b08ac5e7b7bee87e68cf64c7861a81f2ae7713f98bc542ce83904c067c9216c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Staff of the Python

<!-- SRD page 247 -->

Staff, Uncommon (Requires Attunement)\
As a Magic action, you can throw this staff so that it\
lands in an unoccupied space within 10 feet of you,\
causing the staff to become a Giant Constrictor\
Snake in that space. The snake is under your con-\
trol and shares your Initiative count, taking its turn\
immediately after yours.\
On your turn, you can mentally command the\
snake (no action required) if it is within 60 feet of\
you and you don’t have the Incapacitated condition.\
You decide what action the snake takes and where it\
moves during its turn, or you can issue it a general\
command, such as to attack your enemies or guard\
a location. Absent commands from you, the snake\
defends itself.\
As a Bonus Action, you can command the snake\
to revert to staff form in its current space, and you\
can’t use the staff’s property again for 1 hour. If the\
snake is reduced to 0 Hit Points, it dies and reverts\
to its staff form; the staff then shatters and is de-\
stroyed. If the snake reverts to staff form before\
losing all its Hit Points, it regains all of them.\

Original layout: [[../../srd-5.2.1/pages/247|page 247]].
