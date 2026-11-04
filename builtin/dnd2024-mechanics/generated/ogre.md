---
splat_schema: 2
splat_id: 30866ef2-a238-5b10-be76-9bf148aaf82f
splat_kind: creature
splat_key: dnd2024:creatures/ogre
splat_name: Ogre
splat_origin: dnd2024:reference/creature/ogre
splat_sources:
- SRD 5.2.1, pp. 312, https://www.dndbeyond.com/srd
---

# Ogre


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Giant |
| Alignment | Chaotic Evil |
| Armor Class | 11 |
| Hit Points | 68 |
| Initiative | -1 (9) |
| Speed | 40 |
| Challenge |  |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 19 | +4 | +4 |
| Dex | 8 | -1 | -1 |
| Con | 16 | +3 | +3 |
| Int | 5 | -3 | -3 |
| Wis | 7 | -2 | -2 |
| Cha | 7 | -2 | -2 |


<!-- SRD page 312 -->

Large Giant, Chaotic Evil\
AC 11 Initiative −1 (9)\
HP 68 (8d10 + 24)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +4 Dex 8 −1 −1 Con 16 +3 +3\
Int 5 −3 −3 Wis 7 −2 −2 Cha 7 −2 −2\
Gear Greatclub, Javelins (3)\
Senses Darkvision 60 ft.; Passive Perception 8\
Languages Common, Giant\
CR 2 (XP 450; PB +2)

### Actions

Greatclub. Melee Attack Roll: +6, reach 5 ft. Hit: 13 (2d8 + 4) Bludgeoning damage.

Javelin. Melee or Ranged Attack Roll: +6, reach 5 ft. or range 30/120 ft. Hit: 11 (2d6 + 4) Piercing damage.

Original layout: [[../../srd-5.2.1/pages/312|page 312]].

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_actions -->
> 
> ```yaml
> greatclub:
>   name: Greatclub
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
>       modifier: 6
>       scope: attack
>     damage:
>       expression: 2d8 + 4
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
> javelin:
>   name: Javelin (melee)
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
>       modifier: 6
>       scope: attack
>     damage:
>       expression: 2d6 + 4
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
> javelin_ranged:
>   name: Javelin (ranged)
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
>         feet: 30
>         long: 120
>   rolls:
>     attack:
>       expression: 1d20
>       modifier: 6
>       scope: attack
>     damage:
>       expression: 2d6 + 4
>       scope: attack
>   steps:
>   - key: attack
>     operation: dnd2024:attack
>     kind: ranged_weapon
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

