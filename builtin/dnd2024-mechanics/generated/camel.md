---
splat_schema: 2
splat_id: e5d54ebb-04c3-5399-8918-5d36b7621f4b
splat_kind: creature
splat_key: dnd2024:creatures/camel
splat_name: Camel
splat_origin: dnd2024:reference/creature/camel
splat_sources:
- SRD 5.2.1, pp. 346, https://www.dndbeyond.com/srd
---

# Camel


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 10 |
| Hit Points | 17 |
| Initiative | -1 (9) |
| Speed | 50 |
| Challenge |  |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 15 | +2 | +2 |
| Dex | 8 | -1 | -1 |
| Con | 17 | +3 | +5 |
| Int | 2 | -4 | -4 |
| Wis | 11 | +0 | +0 |
| Cha | 5 | -3 | -3 |


<!-- SRD page 346 -->

Large Beast, Unaligned\
AC 10   Initiative −1 (9)\
HP 17 (2d10 + 6)\
Speed 50 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 15 +2 +2 Dex 8 −1 −1 Con 17 +3 +5\
Int 2 −4 −4 Wis 11 +0 +0 Cha 5 −3 −3\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages None\
CR 1/8 (XP 25; PB +2)

### Actions

Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 4 (1d4 + 2) Bludgeoning damage.

Original layout: [[../../srd-5.2.1/pages/346|page 346]].

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

