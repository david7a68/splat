---
splat_schema: 2
splat_id: 91576b90-7b8c-5204-a710-e0f7a3d88545
splat_kind: creature
splat_key: dnd2024:creatures/bat
splat_name: Bat
splat_origin: dnd2024:reference/creature/bat
splat_sources:
- SRD 5.2.1, pp. 345, https://www.dndbeyond.com/srd
---

# Bat


## Stat block

| Stat | Value |
| --- | --- |
| Size | Tiny |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 12 |
| Hit Points | 1 |
| Initiative | +2 (12) |
| Speed | 5 |
| Challenge |  |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 2 | -4 | -4 |
| Dex | 15 | +2 | +2 |
| Con | 8 | -1 | -1 |
| Int | 2 | -4 | -4 |
| Wis | 12 | +1 | +1 |
| Cha | 4 | -3 | -3 |


<!-- SRD page 345 -->

Tiny Beast, Unaligned\
AC 12 Initiative +2 (12)\
HP 1 (1d4 − 1)\
Speed 5 ft., Fly 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 2 −4 −4 Dex 15 +2 +2 Con 8 −1 −1\
Int 2 −4 −4 Wis 12 +1 +1 Cha 4 −3 −3\
Senses Blindsight 60 ft.; Passive Perception 11\
Languages None\
CR 0 (XP 10; PB +2)

### Actions

Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 1 Piercing damage.

Original layout: [[../../srd-5.2.1/pages/345|page 345]].

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_actions -->
> 
> ```yaml
> bite:
>   name: Bite
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
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> fly_speed:
>   value: 30
>   unit: ft
> ```

