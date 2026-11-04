---
splat_schema: 2
splat_id: 4ad4d8a0-ac33-5953-a26d-9f48e8e902f7
splat_kind: creature
splat_key: dnd2024:creatures/guard
splat_name: Guard
splat_origin: dnd2024:reference/creature/guard
splat_sources:
- SRD 5.2.1, creature/guard, https://www.dndbeyond.com/srd
---

# Guard


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Humanoid |
| Armor Class | 16 |
| Hit Points | 11 |
| Initiative | +1 (11) |
| Speed | 30 |
| Challenge |  |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 13 | +1 |  |
| Dex | 12 | +1 |  |
| Con | 12 | +1 |  |
| Int | 10 | +0 |  |
| Wis | 11 | +0 |  |
| Cha | 10 | +0 |  |

%% splat omit=splat_stats/speed/unit %%


Mechanics transcribed from SRD 5.2.1. See the source reference for the complete text.

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_actions -->
> 
> ```yaml
> spear:
>   name: Spear (melee)
>   activation:
>     economy: action
>     category: attack
>     timing: on_turn
>     cost_at: commit
>   targets:
>     chosen:
>       kind: creature
>       count:
>         min: 1
>         max: 1
>       allow_self: false
>       range:
>         feet: 5
>   rolls:
>     attack:
>       expression: 1d20
>       modifier: 3
>       scope: attack
>     damage:
>       expression: 1d6 + 1
>       scope: attack
>   steps:
>   - key: attack
>     operation: dnd2024:attack
>     target: chosen
>     kind: melee_weapon
>     roll: attack
>     on_hit:
>       damage:
>       - type: piercing
>         amount:
>           roll: damage
> throw_spear:
>   name: Spear (ranged)
>   activation:
>     economy: action
>     category: attack
>     timing: on_turn
>     cost_at: commit
>   targets:
>     chosen:
>       kind: creature
>       count:
>         min: 1
>         max: 1
>       allow_self: false
>       range:
>         feet: 20
>         long: 60
>   rolls:
>     attack:
>       expression: 1d20
>       modifier: 3
>       scope: attack
>     damage:
>       expression: 1d6 + 1
>       scope: attack
>   steps:
>   - key: attack
>     operation: dnd2024:attack
>     target: chosen
>     kind: ranged_weapon
>     roll: attack
>     on_hit:
>       damage:
>       - type: piercing
>         amount:
>           roll: damage
> ```
> 
> <!-- splat:yaml path=splat_health -->
> 
> ```yaml
> zero_hp: dnd2024:monster
> ```

