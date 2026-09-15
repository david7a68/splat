---
splat_schema: 1
splat_id: bd98da33-393a-5fcd-8607-15d564659eeb
splat_kind: item
splat_key: dnd2024:items/oil_of_sharpness
splat_name: Oil of Sharpness
splat_origin:
  key: dnd2024:reference/magic_item/oil_of_sharpness
splat_sources:
- title: SRD 5.2.1
  locator: pp. 233
  url: https://www.dndbeyond.com/srd
splat_item:
  category: potion
  rarity: very_rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/oil_of_sharpness
  implementation: dnd2024:item_oil_of_sharpness
  parameters:
    application:
      duration:
        value: 1
        unit: minute
      consume_vial: true
      target_choice:
      - kind: melee_weapon
        count: 1
      - kind: ammunition
        count: 20
      eligibility:
        nonmagical: true
        damage_types_any:
        - slashing
        - piercing
      result:
        melee_weapon:
          definition:
            key: dnd2024:items/weapon_1_2_or_3
          bonus: 3
        ammunition:
          definition:
            key: dnd2024:items/ammunition_1_2_or_3
          bonus: 3
      duration_of_conversion: permanent
    attunement:
      required: false
splat_contracts:
- id: dnd2024:item_oil_of_sharpness
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
    objects:
      required: false
      type: object_list
  outputs:
  - effects
  - item_state
  events:
  - item.used
  capabilities:
  - consumables
  - effects
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 837c11f475cb5c32327544ed115f782c2eaae5ed53f4f4872051b1140f2dbd0d
  source_path: builtin/srd-5.2.1/magic_item/oil_of_sharpness.md
  pages:
  - 233
  engine_gaps: &id001
  - dnd2024:item_oil_of_sharpness
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: 6651db86b8646347aaf754675ddf59249aadc42e845782f0e0178ed9489caba3
splat_clauses:
- key: source_rules
  text: '# Oil of Sharpness Potion, Very Rare One vial of this oil can coat one Melee weapon or twenty
    pieces of ammunition, but only ammunition and Melee weapons that are nonmagical and deal Slashing
    or Piercing damage are affected. Applying the oil takes 1 minute, after which the oil magically seeps
    into whatever it coats, turning the coated weapon into a +3 Weapon or the coated ammunition into +3
    Ammunition. This clear, gelatinous oil sparkles with tiny, ultrathin silver shards.'
  sha256: 88e78a9d1e00defd23f27158a4e7d98a51ac833bc7535e5aecb1f33ac2993e34
  status: contract_specified
splat_requires_mechanics: *id001
---

# Oil of Sharpness

<!-- SRD page 233 -->

Potion, Very Rare\
One vial of this oil can coat one Melee weapon or\
twenty pieces of ammunition, but only ammunition\
and Melee weapons that are nonmagical and deal\
Slashing or Piercing damage are affected. Applying\
the oil takes 1 minute, after which the oil magically\
seeps into whatever it coats, turning the coated\
weapon into a +3 Weapon or the coated ammunition\
into +3 Ammunition.\
This clear, gelatinous oil sparkles with tiny,\
ultrathin silver shards.\

Original layout: [[../../srd-5.2.1/pages/233|page 233]].
