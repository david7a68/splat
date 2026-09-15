---
splat_schema: 1
splat_id: aa54d0e4-b2a7-5377-ba9c-76deb3551e2b
splat_kind: item
splat_key: dnd2024:items/potion_of_longevity
splat_name: Potion of Longevity
splat_origin:
  key: dnd2024:reference/magic_item/potion_of_longevity
splat_sources:
- title: SRD 5.2.1
  locator: pp. 236
  url: https://www.dndbeyond.com/srd
splat_item:
  category: potion
  rarity: very_rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/potion_of_longevity
  implementation: dnd2024:item_potion_of_longevity
  parameters:
    consume: drink
    physical_age_change_years:
      expression: 1d6+6
      scope: use
    default_direction: decrease
    minimum_age_years: 13
    subsequent_use:
      track: lifetime_prior_potions_of_longevity_consumed
      aging_probability_percent:
        op: min
        args:
        - 100
        - op: multiply
          args:
          - 10
          - binding: prior_consumptions
      on_probability_success: increase_age_instead
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
- id: dnd2024:item_potion_of_longevity
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
    prior_consumptions:
      required: false
      type: integer
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
  source_sha256: 94e88c6c041eed2b4c970fb6a1ed7607ea87c1d56fd6e15e05a974814e584adc
  source_path: builtin/srd-5.2.1/magic_item/potion_of_longevity.md
  pages:
  - 236
  engine_gaps: &id001
  - dnd2024:item_potion_of_longevity
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 9a5bbe530e521bbbe4e2dbca6f07c68651fe944efafc32aa802446eeb9f62744
splat_clauses:
- key: source_rules
  text: '# Potion of Longevity Potion, Very Rare When you drink this potion, your physical age is reduced
    by 1d6 + 6 years, to a minimum of 13 years. Each time you subsequently drink a Potion of Longevity,
    there is 10 percent cumulative chance that you instead age by 1d6 + 6 years. Suspended in this amber
    liquid is a tiny heart that, against all reason, is still beating. These ingredients vanish when the
    potion is opened.'
  sha256: 58808d5bc3c73a93d4f63b3389d6914fcfbc1c67bc9bd64490d171af0bb4e4ba
  status: contract_specified
splat_requires_mechanics: *id001
---

# Potion of Longevity

<!-- SRD page 236 -->

Potion, Very Rare\
When you drink this potion, your physical age is\
reduced by 1d6 + 6 years, to a minimum of 13 years.\
Each time you subsequently drink a Potion of Lon-\
gevity, there is 10 percent cumulative chance that\
you instead age by 1d6 + 6 years.\
Suspended in this amber liquid is a tiny heart that,\
against all reason, is still beating. These ingredients\
vanish when the potion is opened.\

Original layout: [[../../srd-5.2.1/pages/236|page 236]].
