---
splat_schema: 1
splat_id: 0d7d38dd-8b58-50e6-969d-1370ad92dcb1
splat_kind: item
splat_key: dnd2024:items/potion_of_healing_superior
splat_name: Potion of Healing (superior)
splat_origin:
  key: dnd2024:reference/magic_item/potions_of_healing
splat_sources:
- title: SRD 5.2.1
  locator: magic_item/potions_of_healing
  url: https://www.dndbeyond.com/srd
splat_item:
  category: consumable
  mass:
    value: 0.5
    unit: lb
  stackable: true
  equipment:
    allowed:
    - carried
    - stored
    attunement: none
splat_actions:
  drink:
    name: Drink or Administer
    activation:
      economy: bonus_action
      category: magic
      timing: on_turn
      cost_at: commit
    requirements:
    - predicate: accessible
      subject: source_item
    - predicate: free_hands
      subject: actor
      minimum: 1
    targets:
      chosen:
        kind: creature
        count:
          min: 1
          max: 1
        allow_self: true
        range:
          feet: 5
    costs:
    - from: source_item
      quantity: 1
      at: resolution
    rolls:
      healing:
        expression: 8d4 + 8
        scope: target
    steps:
    - key: heal
      operation: dnd2024:heal
      target: chosen
      amount:
        roll: healing
---

# Potion of Healing (superior)

Mechanics transcribed from SRD 5.2.1. See the source reference for the complete text.
