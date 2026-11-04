---
splat_schema: 2
splat_id: 5afec75c-7981-5ce1-b7f7-3469de75b409
splat_kind: creature
splat_key: dnd2024:creatures/awakened_shrub
splat_name: Awakened Shrub
splat_origin: dnd2024:reference/creature/awakened_shrub
splat_sources:
- SRD 5.2.1, pp. 260, https://www.dndbeyond.com/srd
---

# Awakened Shrub


## Stat block

| Stat | Value |
| --- | --- |
| Size | Small |
| Type | Plant |
| Alignment | Neutral |
| Armor Class | 9 |
| Hit Points | 10 |
| Initiative | -1 (9) |
| Speed | 20 |
| Challenge |  |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 3 | -4 | -4 |
| Dex | 8 | -1 | -1 |
| Con | 11 | +0 | +0 |
| Int | 10 | +0 | +0 |
| Wis | 10 | +0 | +0 |
| Cha | 6 | -2 | -2 |


<!-- SRD page 260 -->

Small Plant, Neutral\
AC 9 Initiative −1 (9)\
HP 10 (3d6)\
Speed 20 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 3 −4 −4 Dex 8 −1 −1 Con 11 +0 +0\
Int 10 +0 +0 Wis 10 +0 +0 Cha 6 −2 −2\
Vulnerabilities Fire\
Resistances Piercing\
Senses Passive Perception 10\
Languages Common plus one other language\
CR 0 (XP 10; PB +2)

### Actions

Rake. Melee Attack Roll: +1, reach 5 ft. Hit: 1 Slashing damage.

Original layout: [[../../srd-5.2.1/pages/260|page 260]].

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_actions -->
> 
> ```yaml
> rake:
>   name: Rake
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
>       modifier: 1
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
> <!-- splat:yaml path=splat_traits -->
> 
> ```yaml
> damage_affinities:
>   rule: dnd2024:damage_affinities
>   parameters:
>     resistances:
>     - piercing
>     vulnerabilities:
>     - fire
> ```

