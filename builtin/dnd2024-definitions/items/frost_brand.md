---
splat_schema: 1
splat_id: 6b04a7de-44f9-5a60-a461-56889e2ac0d9
splat_kind: item
splat_key: dnd2024:items/frost_brand
splat_name: Frost Brand
splat_origin:
  key: dnd2024:reference/magic_item/frost_brand
splat_sources:
- title: SRD 5.2.1
  locator: pp. 223
  url: https://www.dndbeyond.com/srd
splat_item:
  category: weapon
  rarity: very_rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/frost_brand
  implementation: dnd2024:item_frost_brand
  parameters:
    on_hit:
      extra_damage:
        roll:
          dice: 1
          sides: 6
          bonus: 0
        type: cold
    held:
      resistance:
      - fire
    freezing_temperature:
      light:
        bright_radius_ft: 10
        additional_dim_ft: 10
    draw_weapon:
      optional: true
      extinguish:
        radius_ft: 30
        targets: all_nonmagical_flames
      recovery:
        value: 1
        unit: hour
    attunement:
      required: true
splat_contracts:
- id: dnd2024:item_frost_brand
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
  - item_resources
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: fc2fec8b040d3bd35e88ad824874edd159e9f030bc97dd4c78c403643e3498bb
  source_path: builtin/srd-5.2.1/magic_item/frost_brand.md
  pages:
  - 223
  engine_gaps: &id001
  - dnd2024:item_frost_brand
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: c45ff7025b11bf1b8c28ee7f372307a2e1748dcf481259a00b3efd83d8f23a17
splat_clauses:
- key: source_rules
  text: '# Frost Brand Weapon (Glaive, Greatsword, Longsword, Rapier, Scimitar, or Shortsword), Very Rare
    (Requires Attunement) When you hit with an attack roll using this magic weapon, the target takes an
    extra 1d6 Cold damage. In addition, while you hold the weapon, you have Resistance to Fire damage.
    In freezing temperatures, the weapon sheds Bright Light in a 10-foot radius and Dim Light for an additional
    10 feet. When you draw this weapon, you can extinguish all nonmagical flames within 30 feet of yourself.
    Once used, this property can’t be used again for 1 hour.'
  sha256: 644f452e83bb22ee03b67d8dc7ba7f07439f91b17f00cb6c3c8e911fdc2447b9
  status: contract_specified
splat_requires_mechanics: *id001
---

# Frost Brand

<!-- SRD page 223 -->

Weapon (Glaive, Greatsword, Longsword, Rapier,\
Scimitar, or Shortsword), Very Rare (Requires\
Attunement)\
When you hit with an attack roll using this magic\
weapon, the target takes an extra 1d6 Cold damage.\
In addition, while you hold the weapon, you have\
Resistance to Fire damage.\
In freezing temperatures, the weapon sheds\
Bright Light in a 10-foot radius and Dim Light for an\
additional 10 feet.\
When you draw this weapon, you can extinguish\
all nonmagical flames within 30 feet of yourself.\
Once used, this property can’t be used again for\
1 hour.\

Original layout: [[../../srd-5.2.1/pages/223|page 223]].
