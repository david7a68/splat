---
splat_schema: 1
splat_id: 7e889f70-9118-5d06-ae0c-cbd5d05419de
splat_kind: item
splat_key: dnd2024:items/mace_of_disruption
splat_name: Mace of Disruption
splat_origin:
  key: dnd2024:reference/magic_item/mace_of_disruption
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
  mechanic: dnd2024:items/mace_of_disruption
  implementation: dnd2024:item_mace_of_disruption
  parameters:
    on_hit:
      target_types:
      - fiend
      - undead
      damage:
        amount:
          roll:
            count: 2
            sides: 6
        type: radiant
      after_damage_hp_at_most: 25
      save:
        ability: wisdom
        dc: 15
        failure:
          destroy_target: true
        success:
          condition: frightened
          duration_end: user.next_turn_end
    while_held:
      light:
        bright_radius_ft: 20
        additional_dim_ft: 20
    attunement_required: true
splat_contracts:
- id: dnd2024:item_mace_of_disruption
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
  - attack.hit
  - item.equipment_changed
  - turn.ended
  - item.attunement_completed
  - item.attunement_ended
  capabilities:
  - item_effects
  - dice
  - effect_duration
  - damage
  - conditions
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: ea10bbabe12299b7166416f9a7eb9ac06bbf6a181e6846a4107f1cf07a707406
  source_path: builtin/srd-5.2.1/magic_item/mace_of_disruption.md
  pages:
  - 229
  engine_gaps: &id001
  - dnd2024:item_mace_of_disruption
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 63e741f2507d4e718b6d40d938b4296e139cb82472b8010b66940e9c9b26b1eb
splat_clauses:
- key: source_rules
  text: '# Mace of Disruption Weapon (Mace), Rare (Requires Attunement) When you hit a Fiend or an Undead
    with this magic weapon, that creature takes an extra 2d6 Radiant damage. If the target has 25 Hit
    Points or fewer after taking this damage, it must succeed on a DC 15 Wisdom saving throw or be destroyed.
    On a successful save, the creature has the Frightened condition until the end of your next turn. Light.
    While you hold this weapon, it sheds Bright Light in a 20-foot radius and Dim Light for an additional
    20 feet.'
  sha256: e55d8133909a6df02a7ef7abb6a9bc435d273fdf232a6e35ff8486d96c9c10a3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Mace of Disruption

<!-- SRD page 229 -->

Weapon (Mace), Rare (Requires Attunement)\
When you hit a Fiend or an Undead with this magic\
weapon, that creature takes an extra 2d6 Radiant\
damage. If the target has 25 Hit Points or fewer\
after taking this damage, it must succeed on a DC\
15 Wisdom saving throw or be destroyed. On a suc-\
cessful save, the creature has the Frightened condi-\
tion until the end of your next turn.\
Light. While you hold this weapon, it sheds Bright\
Light in a 20-foot radius and Dim Light for an addi-\
tional 20 feet.\

Original layout: [[../../srd-5.2.1/pages/229|page 229]].
