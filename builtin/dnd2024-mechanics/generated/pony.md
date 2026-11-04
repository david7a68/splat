---
splat_schema: 2
splat_id: 6fad22d3-75bf-5c1d-95a8-e2e367dd4121
splat_kind: creature
splat_key: dnd2024:creatures/pony
splat_name: Pony
splat_origin: dnd2024:reference/creature/pony
splat_sources:
- SRD 5.2.1, pp. 359, https://www.dndbeyond.com/srd
---

# Pony


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 10 |
| Hit Points | 11 |
| Initiative | +0 (10) |
| Speed | 40 |
| Challenge |  |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 15 | +2 | +4 |
| Dex | 10 | +0 | +0 |
| Con | 13 | +1 | +1 |
| Int | 2 | -4 | -4 |
| Wis | 11 | +0 | +0 |
| Cha | 7 | -2 | -2 |


<!-- SRD page 359 -->

Medium Beast, Unaligned\
AC 10  Initiative +0 (10)\
HP 11 (2d8 + 2)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str  15 +2 +4 Dex 10 +0 +0 Con 13 +1 +1\
Int 2 −4 −4 Wis 11 +0 +0 Cha 7 −2 −2\
Senses Passive Perception 10\
Languages None\
CR 1/8 (XP 25; PB +2)

### Actions

Hooves. Melee Attack Roll: +4, reach 5 ft. Hit: 4 (1d4 + 2) Bludgeoning damage.

Original layout: [[../../srd-5.2.1/pages/359|page 359]].

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_actions -->
> 
> ```yaml
> hooves:
>   name: Hooves
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
>       expression: 1d4 + 2
>       scope: attack
>   steps:
>   - key: attack
>     operation: dnd2024:attack
>     kind: melee_weapon
>     target: chosen
>     roll: attack
>     on_hit:
>       damage:
>       - type: bludgeoning
>         amount:
>           roll: damage
> ```
> 
> <!-- splat:yaml path=splat_health -->
> 
> ```yaml
> zero_hp: dnd2024:monster
> ```

