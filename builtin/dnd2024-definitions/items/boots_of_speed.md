---
splat_schema: 1
splat_id: 4c34296c-7b48-55e3-8f65-6ce15204d024
splat_kind: item
splat_key: dnd2024:items/boots_of_speed
splat_name: Boots of Speed
splat_origin:
  key: dnd2024:reference/magic_item/boots_of_speed
splat_sources:
- title: SRD 5.2.1
  locator: pp. 214
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/boots_of_speed
  implementation: dnd2024:item_boots_of_speed
  parameters:
    equipment_state: worn
    activation:
      action: bonus_action
      gesture: click_heels
      speed_multiplier: 2
      opportunity_attacks_against: disadvantage
      end_gesture: click_heels_again
    resource:
      active_duration:
        value: 10
        unit: minute
      scope: wearer
      accumulates_across_activations: true
      on_exhaustion: deactivate
      recovery: wearer_long_rest
    attunement:
      required: true
splat_contracts:
- id: dnd2024:item_boots_of_speed
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
  outputs:
  - effects
  - item_state
  events:
  - item.used
  capabilities:
  - equipment
  - item_resources
  - effects
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: a1724265660d2f1b9e9eea7233042552216cf635f4b5abce62c5f99544746602
  source_path: builtin/srd-5.2.1/magic_item/boots_of_speed.md
  pages:
  - 214
  engine_gaps: &id001
  - dnd2024:item_boots_of_speed
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: ce3a43b87d881168373aff12f7ecb937227b568582eb61725e87b28671de3a19
splat_clauses:
- key: source_rules
  text: '# Boots of Speed Wondrous Item, Rare (Requires Attunement) While you wear these boots, you can
    take a Bonus Action to click the boots’ heels together. If you do, the boots double your Speed, and
    any creature that makes an Opportunity Attack against you has Disadvantage on the attack roll. If
    you click your heels together again, you end the effect. When you’ve used the boots’ property for
    a total of 10 minutes, the magic ceases to function for you until you finish a Long Rest.'
  sha256: 43995462ca61fdbe936134b345b6b1d11b30af94091a9b96a69d59453af7c46d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Boots of Speed

<!-- SRD page 214 -->

Wondrous Item, Rare (Requires Attunement)\
While you wear these boots, you can take a Bonus\
Action to click the boots’ heels together. If you do,\
the boots double your Speed, and any creature that\
makes an Opportunity Attack against you has Dis-\
advantage on the attack roll. If you click your heels\
together again, you end the effect.\
When you’ve used the boots’ property for a total\
of 10 minutes, the magic ceases to function for you\
until you finish a Long Rest.\

Original layout: [[../../srd-5.2.1/pages/214|page 214]].
