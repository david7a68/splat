---
splat_schema: 2
splat_id: 7be357ec-e71b-54e7-974a-3d8acad27110
splat_kind: creature
splat_key: dnd2024:creatures/animated_flying_sword
splat_name: Animated Flying Sword
splat_origin: dnd2024:reference/creature/animated_flying_sword
splat_sources:
- SRD 5.2.1, pp. 259, https://www.dndbeyond.com/srd
---

# Animated Flying Sword


## Stat block

| Stat | Value |
| --- | --- |
| Size | Small |
| Type | Construct |
| Alignment | Unaligned |
| Armor Class | 17 |
| Hit Points | 14 |
| Initiative | +4 (14) |
| Speed | 5 |
| Challenge |  |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 12 | +1 | +1 |
| Dex | 15 | +2 | +4 |
| Con | 11 | +0 | +0 |
| Int | 1 | -5 | -5 |
| Wis | 5 | -3 | -3 |
| Cha | 1 | -5 | -5 |


<!-- SRD page 259 -->

Small Construct, Unaligned\
AC 17 Initiative +4 (14)\
HP 14 (4d6)\
Speed 5 ft., Fly 50 ft. (hover)\
MOD SAVE MOD SAVE MOD SAVE\
Str 12 +1 +1 Dex 15 +2 +4 Con 11 +0 +0\
Int 1 −5 −5 Wis 5 −3 −3 Cha 1 −5 −5\
Immunities Poison, Psychic; Charmed, Deafened, Exhaustion, Frightened, Paralyzed, Petrified, Poisoned\
Senses Blindsight 60 ft.; Passive Perception 7\
Languages None\
CR 1/4 (XP 50; PB +2)

### Actions

Slash. Melee Attack Roll: +4, reach 5 ft. Hit: 6 (1d8 + 2) Slashing damage.

Original layout: [[../../srd-5.2.1/pages/259|page 259]].

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_actions -->
> 
> ```yaml
> slash:
>   name: Slash
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
> 
> <!-- splat:yaml path=splat_traits -->
> 
> ```yaml
> condition_immunities:
>   rule: dnd2024:condition_immunities
>   parameters:
>     conditions:
>     - charmed
>     - deafened
>     - exhaustion
>     - frightened
>     - paralyzed
>     - petrified
>     - poisoned
> damage_affinities:
>   rule: dnd2024:damage_affinities
>   parameters:
>     immunities:
>     - poison
>     - psychic
> ```
> 
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> fly_speed:
>   value: 50
>   unit: ft
> ```

