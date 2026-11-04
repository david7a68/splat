---
splat_schema: 2
splat_id: 7714e47d-87df-57e5-8a58-61c51ede02a6
splat_kind: creature
splat_key: dnd2024:creatures/animated_armor
splat_name: Animated Armor
splat_origin: dnd2024:reference/creature/animated_armor
splat_sources:
- SRD 5.2.1, pp. 259, https://www.dndbeyond.com/srd
---

# Animated Armor


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Construct |
| Alignment | Unaligned |
| Armor Class | 18 |
| Hit Points | 33 |
| Initiative | +2 (12) |
| Speed | 25 |
| Challenge |  |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 14 | +2 | +2 |
| Dex | 11 | +0 | +0 |
| Con | 13 | +1 | +1 |
| Int | 1 | -5 | -5 |
| Wis | 3 | -4 | -4 |
| Cha | 1 | -5 | -5 |


<!-- SRD page 259 -->

Medium Construct, Unaligned\
AC 18 Initiative +2 (12)\
HP 33 (6d8 + 6)\
Speed 25 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 14 +2 +2 Dex 11 +0 +0 Con 13 +1 +1\
Int 1 −5 −5 Wis 3 −4 −4 Cha 1 −5 −5\
Immunities Poison, Psychic; Charmed, Deafened, Exhaustion, Frightened, Paralyzed, Petrified, Poisoned\
Senses Blindsight 60 ft.; Passive Perception 6\
Languages None\
CR 1 (XP 200; PB +2)

### Actions

Multiattack. The armor makes two Slam attacks.

Slam. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2) Bludgeoning damage.

Original layout: [[../../srd-5.2.1/pages/259|page 259]].

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_actions -->
> 
> ```yaml
> slam:
>   name: Slam
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
>       expression: 1d6 + 2
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
>       modifier: 4
>       scope: attack
>     damage:
>       expression: 1d6 + 2
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
>         - type: bludgeoning
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

