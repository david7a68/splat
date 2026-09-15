---
splat_schema: 1
splat_id: 1d6f9e46-bab7-52d0-8377-59a9e84f2098
splat_kind: item
splat_key: dnd2024:items/elixir_of_health
splat_name: Elixir of Health
splat_origin:
  key: dnd2024:reference/magic_item/elixir_of_health
splat_sources:
- title: SRD 5.2.1
  locator: pp. 220
  url: https://www.dndbeyond.com/srd
splat_item:
  category: potion
  rarity: rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/elixir_of_health
  implementation: dnd2024:item_elixir_of_health
  parameters:
    consume: drink
    cure: all_magical_contagions
    end_conditions:
    - blinded
    - deafened
    - paralyzed
    - poisoned
    consume_action: bonus_action
    administer_action: bonus_action
    effect_target: drinker
    consumed_quantity: 1
    effect_starts: immediately_after_consumption
    attunement:
      required: false
    common_rules:
    - key: dnd2024:rules/magic_item_categories_potions
splat_contracts:
- id: dnd2024:item_elixir_of_health
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
  - item.consumed
  capabilities:
  - inventory
  - effects
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: e1f5326f5345885ca80015bc1f63732ce0ce5b74a050c84c19321502ad7a09fb
  source_path: builtin/srd-5.2.1/magic_item/elixir_of_health.md
  pages:
  - 220
  engine_gaps: &id001
  - dnd2024:item_elixir_of_health
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 3fd82845ffbf4dd1a4f507e082c2b925b319aeb5f7fbae7059942294c1069345
splat_clauses:
- key: source_rules
  text: '# Elixir of Health Potion, Rare When you drink this potion, you are cured of all magical contagions.
    In addition, the following conditions end on you: Blinded, Deafened, Paralyzed, and Poisoned. The
    clear, red liquid has tiny bubbles of light in it.'
  sha256: 13409f0fe51985423bf03296de1df15a6fc187d4aef7df3be1eb3b7c1b535e71
  status: contract_specified
splat_requires_mechanics: *id001
---

# Elixir of Health

<!-- SRD page 220 -->

Potion, Rare\
When you drink this potion, you are cured of all\
magical contagions. In addition, the following con-\
ditions end on you: Blinded, Deafened, Paralyzed,\
and Poisoned.\
The clear, red liquid has tiny bubbles of light in it.\

Original layout: [[../../srd-5.2.1/pages/220|page 220]].
