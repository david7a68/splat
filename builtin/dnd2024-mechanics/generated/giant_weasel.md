---
splat_schema: 2
splat_id: 419b96b5-267e-51fa-83d8-2337a760cc3b
splat_kind: creature
splat_key: dnd2024:creatures/giant_weasel
splat_name: Giant Weasel
splat_origin: dnd2024:reference/creature/giant_weasel
splat_sources:
- SRD 5.2.1, pp. 355, https://www.dndbeyond.com/srd
---

# Giant Weasel


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 13 |
| Hit Points | 9 |
| Initiative | +3 (13) |
| Speed | 40 |
| Challenge |  |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 11 | +0 | +0 |
| Dex | 17 | +3 | +3 |
| Con | 10 | +0 | +0 |
| Int | 4 | -3 | -3 |
| Wis | 12 | +1 | +1 |
| Cha | 5 | -3 | -3 |


<!-- SRD page 355 -->

Medium Beast, Unaligned\
AC 13  Initiative +3 (13)\
HP 9 (2d8)\
Speed 40 ft., Climb 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 11 +0 +0 Dex 17 +3 +3 Con 10 +0 +0\
Int 4 −3 −3 Wis 12 +1 +1 Cha 5 −3 −3\
Skills Acrobatics +5, Perception +3, Stealth +5\
Senses Darkvision 60 ft.; Passive Perception 13\
Languages None\
CR 1/8 (XP 25; PB +2)

### Actions

Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 5 (1d4 + 3) Piercing damage.

Original layout: [[../../srd-5.2.1/pages/355|page 355]].

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
>       expression: 1d4 + 3
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
> climb_speed:
>   value: 30
>   unit: ft
> ```

