---
splat_schema: 1
splat_id: 0375c0dd-6a44-4459-88ad-f23d9f156e02
splat_kind: item
splat_key: "my_homebrew:items/dual_wand"
splat_name: Dual Wand
splat_item:
  category: magic_item
  mass: {value: 1, unit: lb}
  stackable: false
  equipment: {allowed: [carried, stored, wielded], hands: 1, attunement: none}
splat_resources:
  fire:
    name: Fire Charges
    max: 4
    recovery:
      - {key: long_rest, mechanic: "dnd2024:long_rest", subject: root_holder, restore: all}
  healing:
    name: Healing Charges
    max: 4
    recovery:
      - {key: short_rest, mechanic: "dnd2024:short_rest", subject: root_holder, restore: {add: 1}}
      - {key: long_rest, mechanic: "dnd2024:long_rest", subject: root_holder, restore: all}
splat_actions:
  flare:
    name: Flare
    activation: {economy: action, category: magic, timing: on_turn, cost_at: commit}
    requirements:
      - {predicate: equipment_state, subject: source_item, state: wielded}
    targets:
      chosen: {kind: creature, count: {min: 1, max: 1}, range: {feet: 30}, visibility: seen}
    costs:
      - {from: source_item, resource: fire, amount: 1, at: commit}
    rolls:
      damage: {expression: "1d6", scope: target}
    steps:
      - key: flare
        operation: "dnd2024:save_damage"
        target: chosen
        ability: dexterity
        dc: 12
        damage: [{type: fire, amount: {roll: damage}}]
        success: {damage_scale: {numerator: 0, denominator: 1}}
  mend:
    name: Wand Mend
    activation: {economy: action, category: magic, timing: on_turn, cost_at: commit}
    requirements:
      - {predicate: equipment_state, subject: source_item, state: wielded}
    targets:
      chosen: {kind: creature, count: {min: 1, max: 1}, allow_self: true, willing: true, range: {touch: true}}
    costs:
      - {from: source_item, resource: healing, amount: 1, at: commit}
    rolls:
      healing: {expression: "1d4 + 1", scope: target}
    steps:
      - {key: healing, operation: "dnd2024:heal", target: chosen, amount: {roll: healing}}
---

Two independent charge types. Rest recovery applies to this item while it is
carried by the participant completing the rest; this is an explicit homebrew
recovery rule, not an assertion about official wands. A standalone item does
not receive another participant's rest events without an applicable rule.

`splat:use flare` · `splat:use mend`
