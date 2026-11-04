---
splat_schema: 2
splat_id: 72cc0187-32ce-54cd-b408-09dc5fb4205c
splat_kind: creature
splat_key: dnd2024:creatures/owlbear
splat_name: Owlbear
splat_origin: dnd2024:reference/creature/owlbear
splat_sources:
- SRD 5.2.1, pp. 313, https://www.dndbeyond.com/srd
---

# Owlbear


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Monstrosity |
| Alignment | Unaligned |
| Armor Class | 13 |
| Hit Points | 59 |
| Initiative | +1 (11) |
| Speed | 40 |
| Challenge |  |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 20 | +5 | +5 |
| Dex | 12 | +1 | +1 |
| Con | 17 | +3 | +3 |
| Int | 3 | -4 | -4 |
| Wis | 12 | +1 | +1 |
| Cha | 7 | -2 | -2 |


<!-- SRD page 313 -->

Large Monstrosity, Unaligned\
AC 13 Initiative +1 (11)\
HP 59 (7d10 + 21)\
Speed 40 ft., Climb 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 20 +5 +5 Dex 12 +1 +1 Con 17 +3 +3\
Int 3 −4 −4 Wis 12 +1 +1 Cha 7 −2 −2\
Skills Perception +5\
Senses Darkvision 60 ft.; Passive Perception 15\
Languages None\
CR 3 (XP 700; PB +2)

### Actions

Multiattack. The owlbear makes two Rend attacks.

Rend. Melee Attack Roll: +7, reach 5 ft. Hit: 14 (2d8 + 5) Slashing damage.

Original layout: [[../../srd-5.2.1/pages/313|page 313]].

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
>       modifier: 7
>       scope: attack
>     damage:
>       expression: 2d8 + 5
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
>       modifier: 7
>       scope: attack
>     damage:
>       expression: 2d8 + 5
>       scope: attack
>   steps:
>   - key: multiattack
>     operation: splat:repeat
>     count: 2
>     steps:
>     - key: attack
>       operation: dnd2024:attack
>       kind: melee_weapon
>       target: chosen
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
> 
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> climb_speed:
>   value: 40
>   unit: ft
> ```

