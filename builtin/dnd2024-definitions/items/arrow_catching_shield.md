---
splat_schema: 1
splat_id: f2ee4569-e383-5e0e-9f73-63624c6494c4
splat_kind: item
splat_key: dnd2024:items/arrow_catching_shield
splat_name: Arrow-Catching Shield
splat_origin:
  key: dnd2024:reference/magic_item/arrow_catching_shield
splat_sources:
- title: SRD 5.2.1
  locator: pp. 211
  url: https://www.dndbeyond.com/srd
splat_item:
  category: armor
  rarity: rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/arrow_catching_shield
  implementation: dnd2024:item_arrow_catching_shield
  parameters:
    equipment_state: wielded
    armor_class:
      bonus: 2
      against: ranged_attack_roll
      stacks_with_normal_shield: true
    redirect:
      action: reaction
      trigger: ranged_attack_roll_against_target_within_5_feet
      target_distance_ft: 5
      replace_target: self
    attunement:
      required: true
splat_contracts:
- id: dnd2024:item_arrow_catching_shield
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
    event:
      required: false
      type: event
  outputs:
  - modifiers
  - effects
  - damage
  - item_state
  events:
  - equipment.state_changed
  - attack.resolved
  - saving_throw.resolved
  capabilities:
  - equipment
  - combat_reactions
  - effects
  - damage
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 3af64da773065a7a70f0b6e22145b5b082f131676c1251b8d56ef2e9b26d0444
  source_path: builtin/srd-5.2.1/magic_item/arrow_catching_shield.md
  pages:
  - 211
  engine_gaps: &id001
  - dnd2024:item_arrow_catching_shield
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: 96840d84ff5adc0093460456ac5f16691b93f2c332222ba3609070c3d312b8ed
splat_clauses:
- key: source_rules
  text: '# Arrow-Catching Shield Armor (Shield), Rare (Requires Attunement) You gain a +2 bonus to Armor
    Class against ranged attack rolls while you wield this Shield. This bonus is in addition to the Shield’s
    normal bonus to AC. Whenever an attacker makes a ranged attack roll against a target within 5 feet
    of you, you can take a Reaction to become the target of the attack instead.'
  sha256: 6d80811eb94dcd70f8fa2bad49d8b0c26dc51804bbb6d09e1b569fe9816334c0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Arrow-Catching Shield

<!-- SRD page 211 -->

Armor (Shield), Rare (Requires Attunement)\
You gain a +2 bonus to Armor Class against ranged\
attack rolls while you wield this Shield. This bonus\
is in addition to the Shield’s normal bonus to AC.\
Whenever an attacker makes a ranged attack roll\
against a target within 5 feet of you, you can take a\
Reaction to become the target of the attack instead.\

Original layout: [[../../srd-5.2.1/pages/211|page 211]].
