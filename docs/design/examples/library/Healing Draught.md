---
splat_schema: 1
splat_id: 0375c0dd-6a44-4459-88ad-f23d9f156e03
splat_kind: item
splat_key: "my_homebrew:items/healing_draught"
splat_name: Healing Draught
splat_item:
  category: consumable
  mass: {value: 0.5, unit: lb}
  stackable: true
  equipment: {allowed: [carried, stored], attunement: none}
splat_actions:
  drink:
    name: Drink or Administer
    activation: {economy: bonus_action, category: magic, timing: on_turn, cost_at: commit}
    requirements:
      - {predicate: accessible, subject: source_item}
      - {predicate: free_hands, subject: actor, minimum: 1}
    targets:
      chosen: {kind: creature, count: {min: 1, max: 1}, allow_self: true, willing: true, range: {feet: 5}}
    costs:
      - {from: source_item, quantity: 1, at: resolution}
    rolls:
      healing: {expression: "2d4 + 2", scope: target}
    steps:
      - {key: healing, operation: "dnd2024:heal", target: chosen, amount: {roll: healing}}
---

A homebrew draught with a Bonus Action use, modeled after the revised potion
timing. Its quantity is consumed when resolution begins; an interruption before
that stage spends the committed Bonus Action but does not consume the draught.

`splat:use drink` · `splat:roll drink.healing`
