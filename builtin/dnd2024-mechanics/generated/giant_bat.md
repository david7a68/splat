---
splat_schema: 2
splat_id: 3e305169-1950-5fa9-b74f-21fa4f6bc77c
splat_kind: creature
splat_key: dnd2024:creatures/giant_bat
splat_name: Giant Bat
splat_origin: dnd2024:reference/creature/giant_bat
splat_sources:
- SRD 5.2.1, pp. 349, https://www.dndbeyond.com/srd
---

# Giant Bat


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 13 |
| Hit Points | 22 |
| Initiative | +3 (13) |
| Speed | 10 |
| Challenge |  |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 15 | +2 | +2 |
| Dex | 16 | +3 | +3 |
| Con | 11 | +0 | +0 |
| Int | 2 | -4 | -4 |
| Wis | 12 | +1 | +1 |
| Cha | 6 | -2 | -2 |


<!-- SRD page 349 -->

Large Beast, Unaligned\
AC 13  Initiative +3 (13)\
HP 22 (4d10)\
Speed 10 ft., Fly 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 15 +2 +2 Dex 16 +3 +3 Con 11 +0 +0\
Int 2 −4 −4 Wis 12 +1 +1 Cha 6 −2 −2\
Senses Blindsight 120 ft.; Passive Perception 11\
Languages None\
CR 1/4 (XP 50; PB +2)

### Actions

Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 6 (1d6 + 3) Piercing damage.

Original layout: [[../../srd-5.2.1/pages/349|page 349]].

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
>       modifier: 5
>       scope: attack
>     damage:
>       expression: 1d6 + 3
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
>   value: 60
>   unit: ft
> ```

