---
splat_schema: 2
splat_id: d56719fd-ca46-59a2-afb6-162639274fc9
splat_kind: creature
splat_key: dnd2024:creatures/draft_horse
splat_name: Draft Horse
splat_origin: dnd2024:reference/creature/draft_horse
splat_sources:
- SRD 5.2.1, pp. 347, 348, https://www.dndbeyond.com/srd
---

# Draft Horse


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 10 |
| Hit Points | 15 |
| Initiative | +0 (10) |
| Speed | 40 |
| Challenge |  |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 18 | +4 | +4 |
| Dex | 10 | +0 | +0 |
| Con | 15 | +2 | +2 |
| Int | 2 | -4 | -4 |
| Wis | 11 | +0 | +0 |
| Cha | 7 | -2 | -2 |


<!-- SRD page 347 -->

Large Beast, Unaligned\
AC 10  Initiative +0 (10)\
HP 15 (2d10 + 4)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 10 +0 +0 Con 15 +2 +2\
Int 2 −4 −4 Wis 11 +0 +0 Cha 7 −2 −2\
Senses Passive Perception 10\
Languages None\
CR 1/4 (XP 50; PB +2)

<!-- SRD page 348 -->

### Actions

Hooves. Melee Attack Roll: +6, reach 5 ft. Hit: 6 (1d4 + 4) Bludgeoning damage.

Original layout: [[../../srd-5.2.1/pages/347|page 347]], [[../../srd-5.2.1/pages/348|page 348]].

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
>       modifier: 6
>       scope: attack
>     damage:
>       expression: 1d4 + 4
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

