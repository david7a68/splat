---
splat_schema: 1
splat_id: bea1d8df-9a89-5e1c-a913-4262061ae9e9
splat_kind: item
splat_key: dnd2024:items/potion_of_giant_strength
splat_name: Potion of Giant Strength
splat_origin:
  key: dnd2024:reference/magic_item/potion_of_giant_strength
splat_sources:
- title: SRD 5.2.1
  locator: pp. 235
  url: https://www.dndbeyond.com/srd
splat_item:
  category: potion
  rarity: rarity_varies
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/potion_of_giant_strength
  implementation: dnd2024:item_potion_of_giant_strength
  parameters:
    trigger: potion_drunk
    consumed: true
    duration:
      amount: 1
      unit: hours
    ability_floor:
      ability: strength
      when: current_score_below_variant
    variants:
    - giant: hill
      score: 21
      rarity: uncommon
    - giant: frost
      score: 23
      rarity: rare
    - giant: stone
      score: 23
      rarity: rare
    - giant: fire
      score: 25
      rarity: rare
    - giant: cloud
      score: 27
      rarity: very_rare
    - giant: storm
      score: 29
      rarity: legendary
    attunement_required: false
splat_contracts:
- id: dnd2024:item_potion_of_giant_strength
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
  - item.consumed
  - time.elapsed
  capabilities:
  - item_effects
  - effect_duration
  - item_variants
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 886d003c758f594862bbf86b6f55695c8eb956a628417212931848a3d967f09f
  source_path: builtin/srd-5.2.1/magic_item/potion_of_giant_strength.md
  pages:
  - 235
  engine_gaps: &id001
  - dnd2024:item_potion_of_giant_strength
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 21c8a4c79c54b6190f3b3fcc7f027ad95f09aaa0bbd24b58414e1899bcb3acbb
splat_clauses:
- key: source_rules
  text: '# Potion of Giant Strength Potion, Rarity Varies When you drink this potion, your Strength score
    changes for 1 hour. The type of giant determines the score (see the table below). The potion has no
    effect on you if your Strength is equal to or greater than that score. This potion’s transparent liquid
    has floating in it a sliver of light resembling a giant’s fingernail. Potion Str. Rarity Potion of
    Giant Strength (hill) 21 Uncommon Potion of Giant Strength (frost or 23 Rare stone) Potion of Giant
    Strength (fire) 25 Rare Potion of Giant Strength (cloud) 27 Very Rare Potion of Giant Strength (storm)
    29 Legendary'
  sha256: 34f58014f2ce03b978b6b3c52cdca298ddc27cfa38279f6494a8908959d8ffa8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Potion of Giant Strength

<!-- SRD page 235 -->

Potion, Rarity Varies\
When you drink this potion, your Strength score\
changes for 1 hour. The type of giant determines the\
score (see the table below). The potion has no effect\
on you if your Strength is equal to or greater than\
that score.\
This potion’s transparent liquid has floating in it a\
sliver of light resembling a giant’s fingernail.\
Potion Str. Rarity\
Potion of Giant Strength (hill) 21 Uncommon\
Potion of Giant Strength (frost or 23 Rare\
stone)\
Potion of Giant Strength (fire) 25 Rare\
Potion of Giant Strength (cloud) 27 Very Rare\
Potion of Giant Strength (storm) 29 Legendary\

Original layout: [[../../srd-5.2.1/pages/235|page 235]].
