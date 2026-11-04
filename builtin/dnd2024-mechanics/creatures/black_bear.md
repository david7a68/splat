---
splat_schema: 2
splat_id: e76f4d97-c2ca-599e-a26e-7ded8834ee22
splat_kind: creature
splat_key: dnd2024:creatures/black_bear
splat_name: Black Bear
splat_origin: dnd2024:reference/creature/black_bear
splat_sources:
- SRD 5.2.1, creature/black_bear, https://www.dndbeyond.com/srd
---

# Black Bear


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Beast |
| Armor Class | 11 |
| Hit Points | 19 |
| Initiative | +1 (11) |
| Speed | 30 |
| Challenge |  |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 15 | +2 |  |
| Dex | 12 | +1 |  |
| Con | 14 | +2 |  |
| Int | 2 | -4 |  |
| Wis | 12 | +1 |  |
| Cha | 7 | -2 |  |

%% splat omit=splat_stats/speed/unit %%


Mechanics transcribed from SRD 5.2.1. See the source reference for the complete text.

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_actions -->
> 
> ```yaml
> rend:
>   name: Rend
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
>       modifier: 4
>       scope: attack
>     damage:
>       expression: 1d6 + 2
>       scope: attack
>   steps:
>   - key: attack
>     operation: dnd2024:attack
>     target: chosen
>     kind: melee_weapon
>     roll: attack
>     on_hit:
>       damage:
>       - type: slashing
>         amount:
>           roll: damage
> multiattack:
>   name: Multiattack
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
>       modifier: 4
>       scope: attack
>     damage:
>       expression: 1d6 + 2
>       scope: attack
>   steps:
>   - key: multiattack
>     operation: splat:repeat
>     count: 2
>     steps:
>     - key: attack
>       operation: dnd2024:attack
>       target: chosen
>       kind: melee_weapon
>       roll: attack
>       on_hit:
>         damage:
>         - type: slashing
>           amount:
>             roll: damage
> ```
> 
> <!-- splat:yaml path=splat_health -->
> 
> ```yaml
> zero_hp: dnd2024:monster
> ```

