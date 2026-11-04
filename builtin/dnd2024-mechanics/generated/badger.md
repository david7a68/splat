---
splat_schema: 2
splat_id: 883a37d4-0528-5c57-a94c-cf5f8e56eb9d
splat_kind: creature
splat_key: dnd2024:creatures/badger
splat_name: Badger
splat_origin: dnd2024:reference/creature/badger
splat_sources:
- SRD 5.2.1, pp. 345, https://www.dndbeyond.com/srd
---

# Badger


## Stat block

| Stat | Value |
| --- | --- |
| Size | Tiny |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 11 |
| Hit Points | 5 |
| Initiative | +0 (10) |
| Speed | 20 |
| Challenge |  |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 10 | +0 | +0 |
| Dex | 11 | +0 | +0 |
| Con | 16 | +3 | +3 |
| Int | 2 | -4 | -4 |
| Wis | 12 | +1 | +1 |
| Cha | 5 | -3 | -3 |


<!-- SRD page 345 -->

Tiny Beast, Unaligned\
AC 11  Initiative +0 (10)\
HP 5 (1d4 + 3)\
Speed 20 ft., Burrow 5 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 10 +0 +0 Dex 11 +0 +0 Con 16 +3 +3\
Int 2 −4 −4 Wis 12 +1 +1 Cha 5 −3 −3\
Skills Perception +3\
Resistances Poison\
Senses Darkvision 30 ft.; Passive Perception 13\
Languages None\
CR 0 (XP 10; PB +2)

### Actions

Bite. Melee Attack Roll: +2, reach 5 ft. Hit: 1 Piercing damage.

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
>       modifier: 2
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
> <!-- splat:yaml path=splat_traits -->
> 
> ```yaml
> damage_affinities:
>   rule: dnd2024:damage_affinities
>   parameters:
>     resistances:
>     - poison
> ```
> 
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> burrow_speed:
>   value: 5
>   unit: ft
> ```

