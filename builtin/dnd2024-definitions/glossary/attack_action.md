---
splat_schema: 1
splat_id: f6d9bfb2-c33a-57d6-982b-847d019de796
splat_kind: rule
splat_key: dnd2024:glossary/attack_action
splat_name: Attack [Action]
splat_origin:
  key: dnd2024:reference/glossary/attack_action
splat_sources:
- title: SRD 5.2.1
  locator: pp. 177
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/attack_action
  parameters:
    cost: action
    default_attacks: 1
    attack_options:
    - weapon
    - unarmed_strike
    equipment_interaction:
      per_attack: 1
      operation:
        chooser: subject
        options:
        - equip
        - unequip
        - none
      timing:
        chooser: subject
        options:
        - before_attack
        - after_attack
      equipped_weapon_must_be_used: false
      equip_methods:
      - draw
      - pick_up
      unequip_methods:
      - sheathe
      - stow
      - drop
    multiple_attacks:
      requires: feature_granted_additional_attacks
      movement_between_attacks: true
      movement_spend: some_or_all_available_turn_movement
  implementation: dnd2024:glossary_attack_action
splat_contracts:
- id: dnd2024:glossary_attack_action
  inputs:
    subject:
      type: participant
      required: true
    targets:
      type: participant_list
      required: true
    weapons:
      required: false
      type: item_list
    attack_plan:
      type: choice
      required: true
  outputs:
  - attacks
  - equipment_changes
  - movement
  events:
  - action.requested
  - attack.pending
  - attack.completed
  capabilities:
  - action_economy
  - attacks
  - equipment
  - movement
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 99d5080b2e7974f6ce53248db4b335dc6a042799b33e3000181638cf1f9bcbe7
  source_path: builtin/srd-5.2.1/glossary/attack_action.md
  pages:
  - 177
  engine_gaps: &id001
  - dnd2024:glossary_attack_action
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 8d7350cd9782a3a4cea5c79fdf862378cfd8738d89fcebd1ae40719778f750e9
splat_clauses:
- key: source_rules
  text: '# Attack [Action] When you take the Attack action, you can make one attack roll with a weapon
    or an Unarmed Strike. Equipping and Unequipping Weapons. You can either equip or unequip one weapon
    when you make an attack as part of this action. You do so either before or after the attack. If you
    equip a weapon before an attack, you don’t need to use it for that attack. Equipping a weapon includes
    drawing it from a sheath or picking it up. Unequipping a weapon includes sheathing, stowing, or dropping
    it. Moving between Attacks. If you move on your turn and have a feature, such as Extra Attack, that
    gives you more than one attack as part of the Attack action, you can use some or all of that movement
    to move between those attacks.'
  sha256: 190f872b566d8be6313a543b368cdb6570be3e288162c6b2ca15974f80a34ba6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Attack [Action]

<!-- SRD page 177 -->

When you take the Attack action, you can make one\
attack roll with a weapon or an Unarmed Strike.\
Equipping and Unequipping Weapons. You can\
either equip or unequip one weapon when you make\
an attack as part of this action. You do so either\
before or after the attack. If you equip a weapon\
before an attack, you don’t need to use it for that at-\
tack. Equipping a weapon includes drawing it from\
a sheath or picking it up. Unequipping a weapon in-\
cludes sheathing, stowing, or dropping it.\
Moving between Attacks. If you move on your\
turn and have a feature, such as Extra Attack, that\
gives you more than one attack as part of the Attack\
action, you can use some or all of that movement to\
move between those attacks.\

Original layout: [[../../srd-5.2.1/pages/177|page 177]].
