---
splat_schema: 1
splat_id: e77448e7-9e61-56e0-b936-83bf1fe2f395
splat_kind: item
splat_key: dnd2024:items/mace_of_smiting
splat_name: Mace of Smiting
splat_origin:
  key: dnd2024:reference/magic_item/mace_of_smiting
splat_sources:
- title: SRD 5.2.1
  locator: pp. 229
  url: https://www.dndbeyond.com/srd
splat_item:
  category: weapon
  rarity: rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/mace_of_smiting
  implementation: dnd2024:item_mace_of_smiting
  parameters:
    weapon_bonus:
      default: 1
      against_construct: 3
      applies:
      - attack
      - damage
    natural_twenty:
      extra_damage:
        type: bludgeoning
        default: 7
        construct: 14
      after_damage:
        target_type: construct
        hp_maximum: 25
        effect: destroy
    attunement:
      required: false
splat_contracts:
- id: dnd2024:item_mace_of_smiting
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
  source_sha256: 1290e820914a135460f1e661d7149e6f63551ada1db6ac6f4aae9aef6c1ad34f
  source_path: builtin/srd-5.2.1/magic_item/mace_of_smiting.md
  pages:
  - 229
  engine_gaps: &id001
  - dnd2024:item_mace_of_smiting
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: e91b5d619f570698805e452f5b5a9b5852a4ca9d020577ca66ea6ff2ec764575
splat_clauses:
- key: source_rules
  text: '# Mace of Smiting Weapon (Mace), Rare You gain a +1 bonus to attack rolls and damage rolls made
    with this magic weapon. The bonus increases to +3 when you use the weapon to attack a Construct. When
    you roll a 20 on an attack roll made with this weapon, the target takes an extra 7 Bludgeoning damage,
    or 14 Bludgeoning damage if it’s a Construct. If a Construct has 25 Hit Points or fewer after taking
    this damage, it is destroyed.'
  sha256: dac25d06eaa68115641e97a6448e858f404f8142d6bcefcedf4ba18a5590f4b7
  status: contract_specified
splat_requires_mechanics: *id001
---

# Mace of Smiting

<!-- SRD page 229 -->

Weapon (Mace), Rare\
You gain a +1 bonus to attack rolls and damage\
rolls made with this magic weapon. The bonus in-\
creases to +3 when you use the weapon to attack a\
Construct.\
When you roll a 20 on an attack roll made with\
this weapon, the target takes an extra 7 Bludgeon-\
ing damage, or 14 Bludgeoning damage if it’s a\
Construct. If a Construct has 25 Hit Points or fewer\
after taking this damage, it is destroyed.\

Original layout: [[../../srd-5.2.1/pages/229|page 229]].
