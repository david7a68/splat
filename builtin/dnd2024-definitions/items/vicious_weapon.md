---
splat_schema: 1
splat_id: e965e3c8-ce7d-571b-b556-2cc1d733703e
splat_kind: item
splat_key: dnd2024:items/vicious_weapon
splat_name: Vicious Weapon
splat_origin:
  key: dnd2024:reference/magic_item/vicious_weapon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 250
  url: https://www.dndbeyond.com/srd
splat_item:
  category: weapon
  rarity: rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/vicious_weapon
  implementation: dnd2024:item_vicious_weapon
  parameters:
    on_hit:
      target_kind: creature
      extra_damage:
        roll:
          dice: 2
          sides: 6
          bonus: 0
        type_from: weapon_normal_damage_type
    attunement:
      required: false
splat_contracts:
- id: dnd2024:item_vicious_weapon
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
  source_sha256: e3f00ac34f1cb2d26d0759bdd4d91c4101611f2bcfd9dbed29f5d7321dd29b38
  source_path: builtin/srd-5.2.1/magic_item/vicious_weapon.md
  pages:
  - 250
  engine_gaps: &id001
  - dnd2024:item_vicious_weapon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: cab20f3cf27201ca2827c4871d85db356c8066dea97d14bcfb549b1cccb8a935
splat_clauses:
- key: source_rules
  text: '# Vicious Weapon Weapon (Any Simple or Martial), Rare This magic weapon deals an extra 2d6 damage
    to any creature it hits. This extra damage is of the same type as the weapon’s normal damage.'
  sha256: a7097ae15b03bd69cb355261763e3a2ba37a94167ae7b0b0cf8066a754185fb9
  status: contract_specified
splat_requires_mechanics: *id001
---

# Vicious Weapon

<!-- SRD page 250 -->

Weapon (Any Simple or Martial), Rare\
This magic weapon deals an extra 2d6 damage to\
any creature it hits. This extra damage is of the\
same type as the weapon’s normal damage.\

Original layout: [[../../srd-5.2.1/pages/250|page 250]].
