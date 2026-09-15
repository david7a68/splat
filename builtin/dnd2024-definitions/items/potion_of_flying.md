---
splat_schema: 1
splat_id: 5ac9961d-7125-5059-baa5-0d8bd2321e82
splat_kind: item
splat_key: dnd2024:items/potion_of_flying
splat_name: Potion of Flying
splat_origin:
  key: dnd2024:reference/magic_item/potion_of_flying
splat_sources:
- title: SRD 5.2.1
  locator: pp. 235
  url: https://www.dndbeyond.com/srd
splat_item:
  category: potion
  rarity: very_rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/potion_of_flying
  implementation: dnd2024:item_potion_of_flying
  parameters:
    consumption:
      trigger: drink
      consume_item: true
    effect:
      duration:
        value: 1
        unit: hour
      fly_speed: walking_speed
      hover: true
      expiry_airborne: fall_unless_other_means_of_staying_aloft
    attunement:
      required: false
splat_contracts:
- id: dnd2024:item_potion_of_flying
  inputs:
    user:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    locations:
      required: false
      type: location_list
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
  source_sha256: d6d31ffee741c3bf497ec60eee482b3436a8607f11654a7ba5dea88eca26249f
  source_path: builtin/srd-5.2.1/magic_item/potion_of_flying.md
  pages:
  - 235
  engine_gaps: &id001
  - dnd2024:item_potion_of_flying
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: f9ed538ae2a03125398229cb85264997c6cf680e3e078c34c88343ceaa18d610
splat_clauses:
- key: source_rules
  text: '# Potion of Flying Potion, Very Rare When you drink this potion, you gain a Fly Speed equal to
    your Speed for 1 hour and can hover. If you’re in the air when the potion wears off, you fall unless
    you have some other means of staying aloft. This potion’s clear liquid floats at the top of its container
    and has cloudy white impurities drifting in it.'
  sha256: 54cd5b23a92d10c473bb26c5a57ebca8a7f57366e30063ca1ef666853ddeddac
  status: contract_specified
splat_requires_mechanics: *id001
---

# Potion of Flying

<!-- SRD page 235 -->

Potion, Very Rare\
When you drink this potion, you gain a Fly Speed\
equal to your Speed for 1 hour and can hover. If\
you’re in the air when the potion wears off, you fall\
unless you have some other means of staying aloft.\
This potion’s clear liquid floats at the top of its\
container and has cloudy white impurities drifting\
in it.\

Original layout: [[../../srd-5.2.1/pages/235|page 235]].
