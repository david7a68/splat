---
splat_schema: 2
splat_id: 5a484d75-2077-5cd5-a2b5-e9a89e725e0e
splat_kind: creature
splat_key: dnd2024:creatures/hawk
splat_name: Hawk
splat_origin: dnd2024:reference/creature/hawk
splat_sources:
- SRD 5.2.1, pp. 355, https://www.dndbeyond.com/srd
---

# Hawk


## Stat block

| Stat | Value |
| --- | --- |
| Size | Tiny |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 13 |
| Hit Points | 1 |
| Initiative | +3 (13) |
| Speed | 10 |
| Challenge |  |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 5 | -3 | -3 |
| Dex | 16 | +3 | +3 |
| Con | 8 | -1 | -1 |
| Int | 2 | -4 | -4 |
| Wis | 14 | +2 | +2 |
| Cha | 6 | -2 | -2 |


<!-- SRD page 355 -->

Tiny Beast, Unaligned\
AC 13  Initiative +3 (13)\
HP 1 (1d4 − 1)\
Speed 10 ft., Fly 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 5 −3 −3 Dex 16 +3 +3 Con 8 −1 −1\
Int 2 −4 −4 Wis 14 +2 +2 Cha 6 −2 −2\
Skills Perception +6\
Senses Passive Perception 16\
Languages None\
CR 0 (XP 10; PB +2)

### Actions

Talons. Melee Attack Roll: +5, reach 5 ft. Hit: 1 Slashing damage.

Original layout: [[../../srd-5.2.1/pages/355|page 355]].

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_actions -->
> 
> ```yaml
> talons:
>   name: Talons
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
>       expression: '1'
>       scope: attack
>   steps:
>   - key: attack
>     operation: dnd2024:attack
>     kind: melee_weapon
>     target: chosen
>     roll: attack
>     on_hit:
>       damage:
>       - type: slashing
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
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> fly_speed:
>   value: 60
>   unit: ft
> ```

