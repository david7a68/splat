---
splat_schema: 2
splat_id: 717bc686-4fb8-5665-b30a-c6a2b4a22fa4
splat_kind: creature
splat_key: dnd2024:creatures/skeleton
splat_name: Skeleton
splat_origin: dnd2024:reference/creature/skeleton
splat_sources:
- SRD 5.2.1, creature/skeleton, https://www.dndbeyond.com/srd
---

# Skeleton


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Undead |
| Armor Class | 14 |
| Hit Points | 13 |
| Initiative | +3 (13) |
| Speed | 30 |
| Challenge |  |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 10 | +0 |  |
| Dex | 16 | +3 |  |
| Con | 15 | +2 |  |
| Int | 6 | -2 |  |
| Wis | 8 | -1 |  |
| Cha | 5 | -3 |  |

%% splat omit=splat_stats/speed/unit %%


Mechanics transcribed from SRD 5.2.1. See the source reference for the complete text.

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_actions -->
> 
> ```yaml
> shortsword:
>   name: Shortsword
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
>       modifier: 5
>       scope: attack
>     damage:
>       expression: 1d6 + 3
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
> shortbow:
>   name: Shortbow
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
>         feet: 80
>         long: 320
>   rolls:
>     attack:
>       expression: 1d20
>       modifier: 5
>       scope: attack
>     damage:
>       expression: 1d6 + 3
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
> 
> <!-- splat:yaml path=splat_traits -->
> 
> ```yaml
> condition_immunities:
>   rule: dnd2024:condition_immunities
>   parameters:
>     conditions:
>     - exhaustion
>     - poisoned
> damage_affinities:
>   rule: dnd2024:damage_affinities
>   parameters:
>     immunities:
>     - poison
>     vulnerabilities:
>     - bludgeoning
> ```

