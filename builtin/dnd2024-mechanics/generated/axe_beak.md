---
splat_schema: 2
splat_id: 3fbef6af-bb6e-51d8-8bf2-9bd632abf451
splat_kind: creature
splat_key: dnd2024:creatures/axe_beak
splat_name: Axe Beak
splat_origin: dnd2024:reference/creature/axe_beak
splat_sources:
- SRD 5.2.1, pp. 260, https://www.dndbeyond.com/srd
---

# Axe Beak


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Monstrosity |
| Alignment | Unaligned |
| Armor Class | 11 |
| Hit Points | 19 |
| Initiative | +1 (11) |
| Speed | 50 |
| Challenge |  |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 14 | +2 | +2 |
| Dex | 12 | +1 | +1 |
| Con | 12 | +1 | +1 |
| Int | 2 | -4 | -4 |
| Wis | 10 | +0 | +0 |
| Cha | 5 | -3 | -3 |


<!-- SRD page 260 -->

Large Monstrosity, Unaligned\
AC 11 Initiative +1 (11)\
HP 19 (3d10 + 3)\
Speed 50 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 14 +2 +2 Dex 12 +1 +1 Con 12 +1 +1\
Int 2 −4 −4 Wis 10 +0 +0 Cha 5 −3 −3\
Senses Passive Perception 10\
Languages None\
CR 1/4 (XP 50; PB +2)

### Actions

Beak. Melee Attack Roll: +4, reach 5 ft. Hit: 6 (1d8 + 2) Slashing damage.

Original layout: [[../../srd-5.2.1/pages/260|page 260]].

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_actions -->
> 
> ```yaml
> beak:
>   name: Beak
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
>       expression: 1d8 + 2
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

