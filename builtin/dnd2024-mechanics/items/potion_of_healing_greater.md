---
splat_schema: 2
splat_id: 099baa87-1236-51af-80e5-66652f639db1
splat_kind: item
splat_key: dnd2024:items/potion_of_healing_greater
splat_name: Potion of Healing (greater)
splat_origin: dnd2024:reference/magic_item/potions_of_healing
splat_sources:
- SRD 5.2.1, magic_item/potions_of_healing, https://www.dndbeyond.com/srd
---

# Potion of Healing (greater)


## Item

| Field | Value |
| --- | --- |
| Type | Consumable |
| Rarity |  |
| Attunement | No |


Mechanics transcribed from SRD 5.2.1. See the source reference for the complete text.

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_actions -->
> 
> ```yaml
> drink:
>   name: Drink or Administer
>   activation:
>     economy: bonus_action
>     category: magic
>     timing: on_turn
>     cost_at: commit
>   requirements:
>   - predicate: accessible
>     subject: source_item
>   - predicate: free_hands
>     subject: actor
>     minimum: 1
>   targets:
>     chosen:
>       kind: creature
>       count:
>         min: 1
>         max: 1
>       allow_self: true
>       range:
>         feet: 5
>   costs:
>   - from: source_item
>     quantity: 1
>     at: resolution
>   rolls:
>     healing:
>       expression: 4d4 + 4
>       scope: target
>   steps:
>   - key: heal
>     operation: dnd2024:heal
>     target: chosen
>     amount:
>       roll: healing
> ```
> 
> <!-- splat:yaml path=splat_item_extras -->
> 
> ```yaml
> equipment:
>   allowed:
>   - carried
>   - stored
> mass:
>   value: 0.5
>   unit: lb
> stackable: true
> ```

